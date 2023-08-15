use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{LitByteStr, LitStr, parse_macro_input, Ident, Token, ItemFn, FnArg, Type, Pat, ReturnType};
use syn::parse::{Parse, ParseStream};

struct Attributes {
    name: Ident,
    topic: MaybeByteStrLit,
}

enum MaybeByteStrLit {
    Str(LitStr),
    Byte(LitByteStr)
}

impl MaybeByteStrLit {
    fn as_bytes_token(&self) -> proc_macro2::TokenStream {
        match self {
            MaybeByteStrLit::Str(lit) => quote!(#lit.as_bytes()),
            MaybeByteStrLit::Byte(lit) => quote!(#lit)
        }
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let topic = input.parse::<LitStr>()
            .map(MaybeByteStrLit::Str)
            .or_else(|_| input.parse::<LitByteStr>().map(MaybeByteStrLit::Byte))?;

        Ok(Attributes {
            name,
            topic
        })
    }
}

struct HandlerArg {
    name: Ident,
    ty: Box<Type>,
}

/// Handler function only allows T: Sized + 'static,
fn allowed_types(ty: &Type) -> bool {
    match ty {
        Type::Array(array) => allowed_types(array.elem.as_ref()),
        Type::Group(group) => allowed_types(group.elem.as_ref()),
        Type::Paren(paren) => allowed_types(paren.elem.as_ref()),
        Type::Path(_) => true,
        Type::Tuple(tuple) => tuple.elems.iter().all(|ty| allowed_types(ty)),
        _ => false
    }
}

#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attributes);
    let func = parse_macro_input!(item as ItemFn);

    let args: Vec<HandlerArg> = func.sig.inputs.iter()
        .map(|input| match input {
            FnArg::Receiver(_) => {
                panic!("cannot use &self/&mut self in handler");
            }
            FnArg::Typed(arg) => {
                if !allowed_types(arg.ty.as_ref()) {
                    panic!("only T: Sized + 'static is allowed as handler arg")
                }
                match arg.pat.as_ref() {
                    Pat::Ident(ident) => HandlerArg {
                        name: ident.ident.to_owned(),
                        ty: arg.ty.to_owned(),
                    },
                    _ => panic!("invalid arg identifier")
                }
            }
        })
        .collect();

    let base_topic = attr.topic.as_bytes_token();
    let service_name = attr.name;
    let request_name = format_ident!("{}Request", service_name);
    let response_name = format_ident!("{}Response", service_name);

    let func_name = func.sig.ident.to_owned();
    let is_async = func.sig.asyncness.is_some();
    if cfg!(feature = "async") && !is_async {
        panic!("synced handler used when async feature enabled");
    } else if !cfg!(feature = "async") && is_async {
        panic!("async handler used when async feature disabled");
    }
    let async_trait = if cfg!(feature = "async") { quote!(#[::comet_eventbus::async_trait]) } else { quote!() };
    let async_token = if cfg!(feature = "async") { quote!(async) } else { quote!() };
    let await_token = if cfg!(feature = "async") { quote!(.await) } else { quote!() };
    let return_type = match func.sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ref ty) => ty.into_token_stream(),
    };

    let args_with_type: Vec<_> = args.iter().map(|HandlerArg { name, ty }| quote!(#name: #ty)).collect();
    let args_only_name: Vec<_> = args.iter().map(|HandlerArg { name, .. }| quote!(#name)).collect();
    let calling_args: Vec<_> = args.iter().map(|HandlerArg { name, .. }| quote!(request.#name)).collect();

    let defines = quote! {
        #[derive(Clone)]
        pub struct #service_name {
            base_topic: ::comet_eventbus::TopicKey,
            eventbus: ::comet_eventbus::Eventbus
        }

        #[derive(Clone)]
        struct #request_name {
            #(#args_with_type,)*
            reply_topic: ::comet_eventbus::TopicKey,
        }

        #[derive(Clone)]
        struct #response_name {
            inner: #return_type,
        }
    };

    let calling = if is_async {
        quote! {
            let inner = #func_name(#(#calling_args,)*).await;
        }
    } else {
        quote! {
            let inner = #func_name(#(#calling_args,)*);
        }
    };

    let listener = quote! {
        #async_trait
        impl ::comet_eventbus::Listener<#request_name> for #service_name {
            #async_token fn handle(&self, event: &::comet_eventbus::Event<#request_name>) -> Result<(), ::comet_eventbus::ListenerError> {
                #func
                let request = event.to_owned().into_inner();
                #calling
                self.eventbus.post(&::comet_eventbus::Event::new(
                    request.reply_topic,
                    #response_name { inner }
                ))#await_token;
                Ok(())
            }
        }
    };

    let implement = quote! {
        impl #service_name {
            pub fn new(eventbus: ::comet_eventbus::Eventbus)-> #service_name {
                Self {
                    base_topic: ::comet_eventbus::TopicKey::from(#base_topic),
                    eventbus
                }
            }

            pub #async_token fn register(&self) {
                self.eventbus.register(self.base_topic.clone(), self.clone())#await_token;
            }

            pub #async_token fn call(
                &self,
                #(#args_with_type,)*
            ) -> #return_type {
                struct OneTimeListener {
                    tx: ::std::sync::Mutex<::std::sync::mpsc::Sender<#return_type>>,
                }

                impl OneTimeListener {
                    fn new() -> (Self, ::std::sync::mpsc::Receiver<#return_type>) {
                        let (tx, rx) = ::std::sync::mpsc::channel();
                        (
                            Self {
                                tx: ::std::sync::Mutex::new(tx)
                            },
                            rx
                        )
                    }
                }

                #async_trait
                impl ::comet_eventbus::Listener<#response_name> for OneTimeListener {
                    #async_token fn handle(&self, event: &::comet_eventbus::Event<#response_name>) -> Result<(), ::comet_eventbus::ListenerError> {
                        let response = event.to_owned().into_inner();
                        self.tx
                            .lock()
                            .unwrap()
                            .send(response.inner)
                            .unwrap();
                        Ok(())
                    }
                }

                let (listener, rx) = OneTimeListener::new();
                let reply_topic = ::comet_eventbus::TopicKey::random(16);
                let request = #request_name {
                    #(#args_only_name,)*
                    reply_topic: reply_topic.clone(),
                };
                let listener = self.eventbus.register(reply_topic.clone(), listener)#await_token;
                self.eventbus.post(&::comet_eventbus::Event::new(reply_topic, request))#await_token;
                let response = rx.recv().unwrap();
                self.eventbus.unregister(listener)#await_token;
                response
            }
        }
    };

    TokenStream::from(quote! {
        #defines

        #listener

        #implement
    })
}
//! Service wrapped of an eventbus.
//!
//! e.g. Create a Handler to accept request and post Response on the topic.


/// expand to :
/// ```
/// #[derive(Clone)]
/// pub struct MyService {
///     eventbus: comet_eventbus::Eventbus
/// }
///
/// struct MyServiceRequest {
///     arg0: u8,
///     arg1: String,
///     arg2: Vec<u8>,
///     reply_topic: comet_eventbus::TopicKey,
/// }
/// struct MyServiceResponse {
///     inner: String,
/// }
///
/// impl Handler<MyServiceRequest> for MyService {
///     fn handle(&self, event: &comet_eventbus::Event<MyServiceRequest>) {
///         fn my_service(arg0: u8, arg1: String, arg2: Vec<u8>) -> String {
///             String::new()
///         }
///         let request = event.into_inner();
///         let inner = my_service(request.arg0, request.arg1, request.arg2);
///         self.eventbus.post(&comet_eventbus::Event::new(
///             request.reply_topic,
///             MyServiceResponse { inner }
///         ));
///     }
/// }
///
/// impl MyService {
///     pub fn new(eventbus: comet_eventbus::Eventbus)-> MyService {
///         Self { eventbus }
///     }
///     pub fn register(&self) {
///         self.eventbus.register("topic", self.clone());
///     }
///     pub fn call(&self, arg0: u8, arg1: String, arg2: Vec<u8>) -> String {
///         struct OneTimeListener {
///             tx: std::sync::mpsc::Sender<String>,
///         }
///
///         impl OneTimeListener {
///             fn new() -> (Self, std::sync::mpsc::Receiver<String>) {
///                 let (tx, rx) = std::sync::mpsc::channel();
///                 (Self { tx }, rx)
///             }
///         }
///
///         impl comet_eventbus::Listener<MyServiceResponse> for OneTimeListener {
///             fn handle(&self, event: &comet_eventbus::Event<MyServiceResponse>) {
///                 let response = event.into_inner();
///                 self.tx.send(response.inner).unwrap();
///             }
///         }
///
///         let (listener, rx) = OneTimeListener::new();
///         let reply_topic = comet_eventbus::TopicKey::random(16);
///         let reply_topic = reply_topic.hash();
///         let request = MyServiceRequest {
///             arg0,
///             arg1,
///             arg2,
///             reply_topic: reply_topic.clone(),
///         };
///         let listener = self.eventbus.register(reply_topic, listener);
///         self.eventbus.post(&comet_eventbus::Event::new(reply_topic, request));
///         let response = rx.recv().unwrap();
///         self.eventbus.unregister(listener);
///         response
///     }
/// }
/// ```
// #[service(MyService, "topic")]
fn my_service(arg0: u8, arg1: String, arg2: Vec<u8>) -> String {
    format!("{}, {}, {:?}", arg0, arg1, arg2)
}
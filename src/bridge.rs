use crate::topic::Topic;
use crate::{Event, EventListener, Eventbus, Listener, TopicKey};
use proto::bridger_server::{Bridger, BridgerServer};
use proto::PostReq;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

type BridgerClient = proto::bridger_client::BridgerClient<tonic::transport::Channel>;

#[allow(unreachable_pub)]
mod proto {
    tonic::include_proto!("bridge");
}

/// A bridge `Topic`
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "bridge")))]
pub struct BridgedTopic<T> {
    inner: Topic<T>,
    bus: EventbusBridge,
}

/// An serialized message
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "bridge")))]
pub struct SerializedMessage {
    inner: Vec<u8>,
}

/// Bridge Serialized Event to an concreate typed Event
///
/// Note: The subscribed topic **MUST** be able to deserialize as type `T`, it **panics**.
/// It's your responsibility to ensure the type is correct.
#[allow(missing_debug_implementations)]
#[cfg_attr(docsrs, doc(cfg(feature = "bridge")))]
pub struct BridgeListener<T> {
    inner: Box<dyn Listener<T>>,
}

/// A bridge to connect two seperated `Eventbus`
#[derive(Debug, Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "bridge")))]
pub struct EventbusBridge {
    bus: Eventbus,
    /// endpoint -> client
    clients: Arc<Mutex<HashMap<String, BridgerClient>>>,
}

/// An `EventListener` wrapper for `Listener`
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "bridge")))]
pub struct BridgedEventListener<T> {
    inner: EventListener<SerializedMessage>,
    bus: EventbusBridge,
    _handler: PhantomData<T>,
}

impl<T> BridgedTopic<T> {
    /// get topic key
    pub fn get_key(&self) -> &TopicKey {
        &self.inner.key
    }
}

impl<T: Serialize + Send + Sync + 'static> BridgedTopic<T> {
    /// shorthand for post event to eventbus
    pub async fn post(&self, event: &Event<T>) {
        self.bus.post(event).await;
    }
}

impl SerializedMessage {
    fn new(inner: Vec<u8>) -> Self {
        Self { inner }
    }
}

impl<T: Serialize> Event<T> {
    /// serialize a message
    pub fn serialized(&self) -> Option<Event<SerializedMessage>> {
        let serialized = bincode::serialize(&self.message).ok()?;
        Some(Event {
            topic: self.topic.clone(),
            message: SerializedMessage::new(serialized),
        })
    }
}

impl Event<SerializedMessage> {
    /// downcast a Serialized Event to a concreate type.
    pub fn downcast<T: Sized + DeserializeOwned + 'static>(&self) -> Option<Event<T>> {
        match bincode::deserialize::<T>(&self.message.inner) {
            Ok(message) => Some(Event {
                topic: self.topic.clone(),
                message,
            }),
            Err(e) => {
                error!("deserialize event message failed: {}", e);
                None
            }
        }
    }
}

impl From<PostReq> for Event<SerializedMessage> {
    fn from(req: PostReq) -> Self {
        Event {
            topic: TopicKey::from(req.topic),
            message: SerializedMessage::new(req.message),
        }
    }
}

impl From<Event<SerializedMessage>> for PostReq {
    fn from(event: Event<SerializedMessage>) -> Self {
        PostReq {
            topic: event.topic.as_ref().to_vec(),
            message: event.message.inner,
        }
    }
}

impl<T> BridgeListener<T> {
    fn new<L: Listener<T>>(listener: L) -> Self {
        Self {
            inner: Box::new(listener),
        }
    }
}

#[async_trait::async_trait]
impl<T: Sized + DeserializeOwned + Send + Sync + 'static> Listener<SerializedMessage>
    for BridgeListener<T>
{
    async fn handle(&self, event: &Event<SerializedMessage>) {
        trace!("handle serialized event of [{:?}]", event.topic);
        let event = event.downcast::<T>().unwrap();
        self.inner.handle(&event).await;
    }
}

#[tonic::async_trait]
impl Bridger for EventbusBridge {
    async fn post(&self, request: Request<PostReq>) -> Result<Response<()>, Status> {
        trace!("recv event from grpc: {:?}", request);
        let req = request.into_inner();
        let event = Event::from(req);
        self.bus.post(&event).await;
        Ok(Response::new(()))
    }
}

impl<T: 'static> BridgedEventListener<T> {
    fn new(inner: EventListener<SerializedMessage>, bus: EventbusBridge) -> Self {
        BridgedEventListener {
            inner,
            bus,
            _handler: Default::default(),
        }
    }

    /// shorthand for unregister bridged listener from eventbus
    pub async fn unregister(self) {
        self.bus.clone().unregister(self).await;
    }
}

impl EventbusBridge {
    /// create a new bridge from an exist `Eventbus`
    pub fn new(bus: Eventbus) -> Self {
        Self {
            bus,
            clients: Arc::new(Default::default()),
        }
    }

    /// connect to another Eventbus
    pub async fn connect<E: AsRef<str>>(&self, endpoint: E) -> Result<(), tonic::transport::Error> {
        let endpoint = endpoint.as_ref().to_string();
        let client = BridgerClient::connect(endpoint.clone()).await?;
        self.clients.lock().await.insert(endpoint, client);
        Ok(())
    }

    /// bind to an address and listen for connections
    pub async fn listen(self, addr: SocketAddr) -> Result<(), tonic::transport::Error> {
        Server::builder()
            .add_service(BridgerServer::new(self))
            .serve(addr)
            .await
    }

    /// create a `Topic` using a topic key
    pub async fn create_topic<T: 'static, K: Into<TopicKey>>(
        &self,
        topic_key: K,
    ) -> BridgedTopic<T> {
        let topic = self.bus.create_topic(topic_key).await;
        BridgedTopic {
            inner: topic,
            bus: self.clone(),
        }
    }

    /// register a listener to bridged eventbus
    pub async fn register<
        T: DeserializeOwned + Send + Sync + 'static,
        K: Into<TopicKey>,
        L: Listener<T>,
    >(
        &self,
        topic_key: K,
        listener: L,
    ) -> BridgedEventListener<T> {
        let listener = BridgeListener::new(listener);
        let inner = self
            .bus
            .register::<SerializedMessage, _, _>(topic_key, listener)
            .await;
        BridgedEventListener::new(inner, self.clone())
    }

    /// unregister a bridged event listener
    pub async fn unregister<T: 'static>(&self, event_listener: BridgedEventListener<T>) {
        self.bus.unregister(event_listener.inner).await;
    }

    /// post an event to eventbus
    pub async fn post<T: Serialize + Send + Sync + 'static>(&self, event: &Event<T>) {
        let serialized: PostReq = event.serialized().unwrap().into();
        let mut guard = self.clients.lock().await;
        tokio::join!(
            self.bus.post(event),
            futures::future::join_all(
                guard
                    .iter_mut()
                    .map(|(_, client)| client.post(serialized.clone()))
            )
        );
    }
}

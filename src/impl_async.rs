use async_trait::async_trait;
use futures::future;
use crate::{Event, Eventbus, EventListener, EventListeners, Topic, TopicHandlers, TopicHandlersMap, TopicKey};

/// Event listener
///
/// Note: the struct which implements `Listener` need to be `Send` and `Sync`
#[async_trait]
pub trait Listener<T>: Send + Sync + 'static {
    /// handler callback to process event
    async fn handle(&self, _: &Event<T>);
}

impl Eventbus {
    /// create a `Topic` using a topic key
    pub async fn create_topic<T: 'static, K: Into<TopicKey>>(&self, topic_key: K) -> Topic<T> {
        let topic_key = topic_key.into();
        let listeners = self
            .inner
            .topic_handlers
            .get_listener(topic_key.clone())
            .await;
        Topic {
            key: topic_key,
            bus: self.clone(),
            event_listeners: listeners,
        }
    }

    /// register a listener to eventbus
    pub async fn register<T: 'static>(&self, listener: EventListener<T>) {
        self.inner.topic_handlers.add_listener(listener).await;
    }

    /// post an event to eventbus
    pub async fn post<T: Send + Sync + 'static>(&self, event: &Event<T>) {
        self.inner.topic_handlers.notify(event).await;
    }
}

impl TopicHandlers {
    async fn add_listener<T: 'static>(&self, listener: EventListener<T>) {
        let listeners = self.get_listener(listener.topic.clone()).await;
        listeners.lock().await.insert(listener);
    }

    async fn get_listener<T: 'static, K: Into<TopicKey>>(&self, topic_key: K) -> EventListeners<T> {
        let mut guard = self.inner.lock().await;
        if !guard.contains::<TopicHandlersMap<T>>() {
            guard.insert::<TopicHandlersMap<T>>(Default::default());
        }
        let inner = guard.get::<TopicHandlersMap<T>>().unwrap();
        let mut inner_guard = inner.lock().await;

        let topic_key = topic_key.into();
        let listeners = inner_guard
            .entry(topic_key)
            .or_insert_with(Default::default);
        listeners.clone()
    }

    async fn notify<T: Send + Sync + 'static>(&self, event: &Event<T>) {
        let listeners = self.get_listener::<T, _>(event.topic.clone()).await;
        let guard = listeners.lock().await;
        future::join_all(guard.iter().map(|listener| listener.handler.handle(event))).await;
    }
}

impl<T: Send + Sync + 'static> Topic<T> {
    /// shorthand for post event to eventbus
    pub async fn post(&self, event: &Event<T>) {
        self.bus.post(event).await;
    }
}
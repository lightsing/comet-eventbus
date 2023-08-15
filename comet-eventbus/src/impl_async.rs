use crate::{
    Event, EventListener, EventListeners, Eventbus, ListenerError, Topic, TopicHandlers,
    TopicHandlersMap, TopicKey,
};
use async_trait::async_trait;
use futures::future;

/// Event listener
///
/// Note: the struct which implements `Listener` need to be `Send` and `Sync`
#[async_trait]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub trait Listener<T>: Send + Sync + 'static {
    /// handler callback to process event
    async fn handle(&self, _: &Event<T>) -> Result<(), ListenerError>;
}

impl Eventbus {
    /// create a `Topic` using a topic key
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
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
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub async fn register<T: 'static, K: Into<TopicKey>, L: Listener<T>>(
        &self,
        topic_key: K,
        listener: L,
    ) -> EventListener<T> {
        let topic_key = topic_key.into();
        let event_listener = EventListener::<T>::new(topic_key.clone(), self.clone());
        trace!("add event_listener: {:?}", event_listener);
        self.inner
            .topic_handlers
            .add_listener(event_listener.rand_id, topic_key, listener)
            .await;
        event_listener
    }

    /// unregister an event listener
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub async fn unregister<T: 'static>(&self, event_listener: EventListener<T>) {
        self.inner
            .topic_handlers
            .remove_listener::<T, _>(event_listener.rand_id, event_listener.topic)
            .await;
    }

    /// post an event to eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub async fn post<T: Send + Sync + 'static>(&self, event: &Event<T>) {
        trace!("recv post [{:?}]", event.topic);
        self.inner.topic_handlers.notify(event).await;
    }
}

impl<T: 'static> EventListener<T> {
    /// shorthand for unregister listener from eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub async fn unregister(self) {
        self.bus.clone().unregister(self).await
    }
}

impl TopicHandlers {
    async fn add_listener<T: 'static, K: Into<TopicKey>, L: Listener<T>>(
        &self,
        rand_id: u64,
        topic_key: K,
        listener: L,
    ) {
        trace!("add listener: rand_id={}", rand_id);
        let listeners = self.get_listener::<T, K>(topic_key).await;
        listeners.lock().await.insert(rand_id, Box::new(listener));
    }

    async fn remove_listener<T: 'static, K: Into<TopicKey>>(&self, rand_id: u64, topic_key: K) {
        let listeners = self.get_listener::<T, K>(topic_key).await;
        listeners.lock().await.remove(&rand_id);
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
        trace!("current listeners: {}", listeners.lock().await.len());
        listeners.clone()
    }

    async fn notify<T: Send + Sync + 'static>(&self, event: &Event<T>) {
        let listeners = self.get_listener::<T, _>(event.topic.clone()).await;
        let guard = listeners.lock().await;
        future::join_all(guard.iter().map(|(_, listener)| {
            trace!("notify listener for event [{:?}]", event.topic);
            async {
                let result = listener.handle(event).await;
                if let Err(e) = result {
                    error!(
                        "listener of topic [{}] failed to process event: {:?}",
                        event.topic, e
                    )
                }
            }
        }))
        .await;
    }
}

impl<T: Send + Sync + 'static> Topic<T> {
    /// shorthand for post event to eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub async fn post(&self, event: &Event<T>) {
        self.bus.post(event).await;
    }

    /// shorthand for post message to eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub async fn post_message(&self, message: T) {
        let event = self.create_event(message);
        self.post(&event).await;
    }
}

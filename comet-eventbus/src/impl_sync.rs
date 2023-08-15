use crate::{
    Event, EventListener, EventListeners, Eventbus, ListenerError, Topic, TopicHandlers,
    TopicHandlersMap, TopicKey,
};
#[cfg(feature = "sync_parallel")]
use rayon::prelude::*;

/// Event listener
///
/// Note: the struct which implements `Listener` need to be `Send` and `Sync`
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub trait Listener<T>: Send + Sync + 'static {
    /// handler callback to process event
    fn handle(&self, _: &Event<T>) -> Result<(), ListenerError>;
}

impl Eventbus {
    /// create a `Topic` using a topic key
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn create_topic<T: 'static, K: Into<TopicKey>>(&self, topic_key: K) -> Topic<T> {
        let topic_key = topic_key.into();
        let listeners = self.inner.topic_handlers.get_listener(topic_key.clone());
        Topic {
            key: topic_key,
            bus: self.clone(),
            event_listeners: listeners,
        }
    }

    /// register a listener to eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn register<T: 'static, K: Into<TopicKey>, L: Listener<T>>(
        &self,
        topic_key: K,
        listener: L,
    ) -> EventListener<T> {
        let topic_key = topic_key.into();
        let event_listener = EventListener::<T>::new(topic_key.clone(), self.clone());
        trace!("add event_listener: {:?}", event_listener);
        self.inner
            .topic_handlers
            .add_listener(event_listener.rand_id, topic_key, listener);
        event_listener
    }

    /// unregister an event listener
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn unregister<T: 'static>(&self, event_listener: EventListener<T>) {
        self.inner
            .topic_handlers
            .remove_listener::<T, _>(event_listener.rand_id, event_listener.topic);
    }

    /// post an event to eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn post<T: Sync + 'static>(&self, event: &Event<T>) {
        self.inner.topic_handlers.notify(event);
    }
}

impl<T: 'static> EventListener<T> {
    /// shorthand for unregister listener from eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn unregister(self) {
        self.bus.clone().unregister(self)
    }
}

impl TopicHandlers {
    fn add_listener<T: 'static, K: Into<TopicKey>, L: Listener<T>>(
        &self,
        rand_id: u64,
        topic_key: K,
        listener: L,
    ) {
        trace!("add listener: rand_id={}", rand_id);
        let listeners = self.get_listener::<T, K>(topic_key);
        listeners.lock().insert(rand_id, Box::new(listener));
    }

    fn remove_listener<T: 'static, K: Into<TopicKey>>(&self, rand_id: u64, topic_key: K) {
        let listeners = self.get_listener::<T, K>(topic_key);
        listeners.lock().remove(&rand_id);
    }

    fn get_listener<T: 'static, K: Into<TopicKey>>(&self, topic_key: K) -> EventListeners<T> {
        let mut guard = self.inner.lock();
        if !guard.contains::<TopicHandlersMap<T>>() {
            guard.insert::<TopicHandlersMap<T>>(Default::default());
        }
        let inner = guard.get::<TopicHandlersMap<T>>().unwrap();
        let mut inner_guard = inner.lock();

        let topic_key = topic_key.into();
        let listeners = inner_guard
            .entry(topic_key)
            .or_insert_with(Default::default);
        trace!("current listeners: {}", listeners.lock().len());
        listeners.clone()
    }

    fn notify<T: Sync + 'static>(&self, event: &Event<T>) {
        let listeners = self.get_listener::<T, _>(event.topic.clone());
        let guard = listeners.lock();

        #[cfg(not(feature = "sync_parallel"))]
        guard.iter().for_each(|(_, listener)| {
            trace!("notify listener for event [{:?}]", event.topic);
            if let Err(e) = listener.handle(event) {
                error!(
                    "listener of topic [{}] failed to process event: {:?}",
                    event.topic, e
                )
            }
        });

        #[cfg(feature = "sync_parallel")]
        guard.par_iter().for_each(|(_, listener)| {
            trace!("notify listener for event [{:?}]", event.topic);
            if let Err(e) = listener.handle(event) {
                error!(
                    "listener of topic [{}] failed to process event: {:?}",
                    event.topic, e
                )
            }
        });
    }
}

impl<T: Sync + 'static> Topic<T> {
    /// shorthand for post event to eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn post(&self, event: &Event<T>) {
        self.bus.post(event);
    }

    /// shorthand for post message to eventbus
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn post_message(&self, message: T) {
        let event = self.create_event(message);
        self.post(&event);
    }
}

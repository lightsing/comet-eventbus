#[cfg(feature = "sync_parallel")]
use rayon::prelude::*;
use crate::{Event, Eventbus, EventListener, EventListeners, Topic, TopicHandlers, TopicHandlersMap, TopicKey};

/// Event listener
///
/// Note: the struct which implements `Listener` need to be `Send` and `Sync`
pub trait Listener<T>: Sync + 'static {
    /// handler callback to process event
    fn handle(&self, _: &Event<T>);
}

impl Eventbus {
    /// create a `Topic` using a topic key
    pub fn create_topic<T: 'static, K: Into<TopicKey>>(&self, topic_key: K) -> Topic<T> {
        let topic_key = topic_key.into();
        let listeners = self
            .inner
            .topic_handlers
            .get_listener(topic_key.clone());
        Topic {
            key: topic_key,
            bus: self.clone(),
            event_listeners: listeners,
        }
    }

    /// register a listener to eventbus
    pub fn register<T: 'static>(&self, listener: EventListener<T>) {
        self.inner.topic_handlers.add_listener(listener);
    }

    /// post an event to eventbus
    pub fn post<T: Sync + 'static>(&self, event: &Event<T>) {
        self.inner.topic_handlers.notify(event);
    }
}

impl TopicHandlers {
    fn add_listener<T: 'static>(&self, listener: EventListener<T>) {
        let listeners = self.get_listener(listener.topic.clone());
        listeners.lock().insert(listener);
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
        listeners.clone()
    }

    fn notify<T: Sync + 'static>(&self, event: &Event<T>) {
        let listeners = self.get_listener::<T, _>(event.topic.clone());
        let guard = listeners.lock();

        #[cfg(not(feature = "sync_parallel"))]
        guard.iter().for_each(|listener| listener.handler.handle(event));

        #[cfg(feature = "sync_parallel")]
        guard.par_iter().for_each(|listener| listener.handler.handle(event));
    }
}

impl<T: Sync + 'static> Topic<T> {
    /// shorthand for post event to eventbus
    pub fn post(&self, event: &Event<T>) {
        self.bus.post(event);
    }
}
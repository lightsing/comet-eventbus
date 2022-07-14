//! An universal eventbus for Rust!
//!
//! This crate provides a strong-typed asynchronous eventbus implementation.

#![deny(missing_docs)]
#![warn(
    missing_debug_implementations,
    single_use_lifetimes,
    unreachable_pub,
    future_incompatible,
    rust_2021_compatibility,
)]

use anymap::AnyMap;
use async_trait::async_trait;
use futures::future;
use rand::{thread_rng, RngCore};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::Utf8Error;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(test)]
mod tests;

/// An asynchronous `Eventbus` to interact with
#[derive(Debug, Clone)]
pub struct Eventbus {
    inner: Arc<EventbusInner>,
}

/// Wrapper of bytes represent a `Topic`
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TopicKey(Vec<u8>);

/// short hand of event listeners set
pub type EventListeners<T> = Arc<Mutex<HashSet<EventListener<T>>>>;
/// short hand of topic to handlers map
pub type TopicHandlersMap<T> = Arc<Mutex<HashMap<TopicKey, EventListeners<T>>>>;

/// Event listener
///
/// Note: the struct which implements `Listener` need to be `Send` and `Sync`
#[async_trait]
pub trait Listener<T>: Send + Sync + 'static {
    /// handler callback to process event
    async fn handle(&self, _: &Event<T>);
}

/// A `Topic` wrapper for a `TopicKey`
#[derive(Debug)]
pub struct Topic<T> {
    key: TopicKey,
    bus: Eventbus,
    event_listeners: EventListeners<T>,
}

/// An `Event` for passing
pub struct Event<T> {
    topic: TopicKey,
    message: T,
}

#[derive(Debug)]
struct EventbusInner {
    topic_handlers: Arc<TopicHandlers>,
}

/// An `EventListener` wrapper for `Listener`
pub struct EventListener<T> {
    topic: TopicKey,
    rand_id: u64,
    handler: Box<dyn Listener<T>>,
}

#[derive(Debug)]
struct TopicHandlers {
    inner: Mutex<AnyMap>,
}

impl Eventbus {
    /// create an new eventbus
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EventbusInner {
                topic_handlers: Arc::new(TopicHandlers::new())
            })
        }
    }

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
    fn new() -> Self {
        Self {
            inner: Mutex::new(AnyMap::new())
        }
    }

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
            .entry(topic_key.clone())
            .or_insert_with(|| Default::default());
        listeners.clone()
    }

    async fn notify<T: Send + Sync + 'static>(&self, event: &Event<T>) {
        let listeners = self.get_listener::<T, _>(event.topic.clone()).await;
        let guard = listeners.lock().await;
        future::join_all(guard.iter().map(|listener| listener.handler.handle(&event))).await;
    }
}

impl<T> Event<T> {
    /// create an new event
    pub fn new<K: Into<TopicKey>>(topic_key: K, message: T) -> Self {
        Self {
            topic: topic_key.into(),
            message
        }
    }
}

impl<T: Send + Sync + 'static> Topic<T> {
    /// shorthand for post event to eventbus
    pub async fn post(&self, event: &Event<T>) {
        self.bus.post(event).await;
    }
}

impl<T> Topic<T> {
    /// get the key of a topic
    pub fn get_key(&self) -> &TopicKey {
        &self.key
    }

    /// get the associated eventbus
    pub fn get_bus(&self) -> &Eventbus {
        &self.bus
    }

    /// get event listeners subscribed to this topic
    pub fn get_listeners(&self) -> &EventListeners<T> {
        &self.event_listeners
    }
}

impl<T> EventListener<T> {
    /// create a `EventListener` from a handler and a topic key
    pub fn new<K: Into<TopicKey>, H: Listener<T>>(topic_key: K, handler: H) -> EventListener<T> {
        EventListener {
            topic: topic_key.into(),
            rand_id: thread_rng().next_u64(),
            handler: Box::new(handler),
        }
    }
}

impl TopicKey {
    /// try parse topic key as an utf-8 str
    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.0)
    }
}

impl Display for TopicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.try_as_str().unwrap_or(&hex::encode(&self.0)))
    }
}

impl Debug for TopicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f
            .debug_tuple("TopicKey")
            .field(&self.try_as_str().unwrap_or(&hex::encode(&self.0)))
            .finish()
    }
}

impl<B> From<B> for TopicKey
    where
        B: AsRef<[u8]>,
{
    fn from(value: B) -> Self {
        Self(value.as_ref().to_vec())
    }
}

impl <T: Debug> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct(&format!("Event<{}>", std::any::type_name::<T>()).as_str())
            .field("topic", &self.topic)
            .field("message", &self.message)
            .finish()
    }
}

impl<T: Clone> Clone for Event<T> {
    fn clone(&self) -> Self {
        Self {
            topic: self.topic.clone(),
            message: self.message.clone(),
        }
    }
}

impl<T> PartialEq<Self> for EventListener<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rand_id.eq(&other.rand_id) && self.topic.eq(&other.topic)
    }
}

impl<T> Eq for EventListener<T> {}

impl<T> Hash for EventListener<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.topic.hash(state);
        state.write_u64(self.rand_id);
    }
}

impl<T> Debug for EventListener<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct(&format!("EventListener<{}>", std::any::type_name::<T>()).as_str())
            .field("topic", &self.topic)
            .field("rand_id", &self.rand_id)
            .finish()
    }
}

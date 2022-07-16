//! An universal eventbus for Rust!
//!
//! This crate provides a strong-typed asynchronous eventbus implementation.

#![deny(missing_docs)]
#![warn(
    missing_debug_implementations,
    single_use_lifetimes,
    unreachable_pub,
    future_incompatible,
    rust_2021_compatibility
)]

#[macro_use]
extern crate log;

use rand::{thread_rng, RngCore};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::str::Utf8Error;
use std::sync::Arc;

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!("Either `async` or `sync` feature must be enabled");

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("The `async` and `sync` features cannot be enabled simultaneously");

#[cfg(feature = "async")]
pub use async_trait::async_trait;

/// bridge `Eventbus` from an external source
#[cfg(feature = "bridge")]
pub mod bridge;
#[cfg(feature = "async")]
mod impl_async;
#[cfg(feature = "sync")]
mod impl_sync;
#[cfg(test)]
mod tests;

#[cfg(feature = "async")]
pub use impl_async::Listener;
#[cfg(feature = "sync")]
pub use impl_sync::Listener;

#[cfg(feature = "sync")]
use parking_lot::Mutex;
#[cfg(feature = "async")]
use tokio::sync::Mutex;

/// An asynchronous `Eventbus` to interact with
#[derive(Debug, Clone)]
pub struct Eventbus {
    inner: Arc<EventbusInner>,
}

/// Wrapper of bytes represent a `Topic`
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TopicKey(Vec<u8>);

/// short hand of event listeners set
pub type EventListeners<T> = Arc<Mutex<HashMap<u64, Box<dyn Listener<T>>>>>;
/// short hand of topic to handlers map
pub type TopicHandlersMap<T> = Arc<Mutex<HashMap<TopicKey, EventListeners<T>>>>;

/// A `Topic` wrapper for a `TopicKey`
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
    bus: Eventbus,
    _handler: PhantomData<T>,
}

#[derive(Debug)]
struct TopicHandlers {
    inner: Mutex<anymap::Map<dyn anymap::any::Any + Send + Sync>>,
}

impl Eventbus {
    /// create an new eventbus
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EventbusInner {
                topic_handlers: Arc::new(TopicHandlers::new()),
            }),
        }
    }
}

impl Default for Eventbus {
    fn default() -> Self {
        Self::new()
    }
}

impl TopicHandlers {
    fn new() -> Self {
        Self {
            inner: Mutex::new(anymap::Map::new()),
        }
    }
}

impl<T> Event<T> {
    /// create an new event
    pub fn new<K: Into<TopicKey>>(topic_key: K, message: T) -> Self {
        Self {
            topic: topic_key.into(),
            message,
        }
    }

    /// into inner message
    pub fn into_inner(self) -> T {
        self.message
    }
}

impl<T> Deref for Event<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.message
    }
}

impl<T> DerefMut for Event<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.message
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

impl<T> Debug for Topic<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("Topic<{}>", std::any::type_name::<T>()).as_str())
            .field("key", &self.key)
            .field("bus", &self.bus)
            .finish()
    }
}

impl<T> EventListener<T> {
    fn new<K: Into<TopicKey>>(topic_key: K, bus: Eventbus) -> EventListener<T> {
        EventListener {
            topic: topic_key.into(),
            rand_id: thread_rng().next_u64(),
            bus,
            _handler: PhantomData,
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
        f.debug_tuple("TopicKey")
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

impl<T: Debug> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("Event<{}>", std::any::type_name::<T>()).as_str())
            .field("topic", &self.topic)
            .field("message", &&self.message)
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

impl<T> Clone for EventListener<T> {
    fn clone(&self) -> Self {
        Self {
            topic: self.topic.clone(),
            rand_id: self.rand_id,
            bus: self.bus.clone(),
            _handler: PhantomData,
        }
    }
}

impl<T> Debug for EventListener<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("EventListener<{}>", std::any::type_name::<T>()).as_str())
            .field("topic", &self.topic)
            .field("rand_id", &self.rand_id)
            .finish()
    }
}

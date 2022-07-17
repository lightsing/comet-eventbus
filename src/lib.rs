//! An universal eventbus for Rust!
//!
//! This crate provides a strong-typed asynchronous eventbus implementation.
//!
//! # Get Started
//!
//! comet-eventbus is async-first-classed. We recommend you to use async API.
//!
//! Add following code to your `Cargo.toml`:
//! ```toml
//! comet-eventbus = "0.1"
//! ```
//!
//! ## Example
//!
//! ```
//! use comet_eventbus::{Event, Eventbus};
//!
//! // define your message struct
//! struct Message {
//!     content: u8,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // creat a new eventbus
//!     let eventbus = Eventbus::new();
//!
//!     // create topic
//!     let topic = eventbus.create_topic("my awsome topic").await;
//!
//!     // post message to a topic
//!     topic.post_message(Message { content: 0 }).await;
//! }
//! ```
//!
#![deny(missing_docs)]
#![warn(
    missing_debug_implementations,
    single_use_lifetimes,
    unreachable_pub,
    future_incompatible,
    rust_2021_compatibility
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!("Either `async` or `sync` feature must be enabled");

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("The `async` and `sync` features cannot be enabled simultaneously");

#[cfg(feature = "async")]
pub use async_trait::async_trait;

/// bridge `Eventbus` from an external source
#[cfg(feature = "bridge")]
#[cfg_attr(docsrs, doc(cfg(feature = "bridge")))]
pub mod bridge;
mod event;
mod event_listener;
#[cfg(feature = "async")]
mod impl_async;
#[cfg(feature = "sync")]
mod impl_sync;
#[cfg(test)]
mod tests;
mod topic;
mod topic_key;

pub use event::Event;
pub use event_listener::EventListener;
pub use topic::Topic;
pub use topic_key::TopicKey;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use impl_async::Listener;
#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
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

/// short hand of event listeners set
pub type EventListeners<T> = Arc<Mutex<HashMap<u64, Box<dyn Listener<T>>>>>;
/// short hand of topic to handlers map
pub type TopicHandlersMap<T> = Arc<Mutex<HashMap<TopicKey, EventListeners<T>>>>;

#[derive(Debug)]
struct EventbusInner {
    topic_handlers: Arc<TopicHandlers>,
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

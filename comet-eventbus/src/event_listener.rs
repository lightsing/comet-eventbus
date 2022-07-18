use crate::{Eventbus, TopicKey};
use rand::{thread_rng, RngCore};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// An `EventListener` wrapper for `Listener`
pub struct EventListener<T> {
    pub(crate) topic: TopicKey,
    pub(crate) rand_id: u64,
    pub(crate) bus: Eventbus,
    _handler: PhantomData<T>,
}

impl<T> EventListener<T> {
    pub(crate) fn new<K: Into<TopicKey>>(topic_key: K, bus: Eventbus) -> EventListener<T> {
        EventListener {
            topic: topic_key.into(),
            rand_id: thread_rng().next_u64(),
            bus,
            _handler: PhantomData,
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

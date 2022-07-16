use comet_eventbus::*;
use log::info;
/// cargo run --example sync --no-default-features --features sync
use std::ops::Deref;

struct Handler;

#[derive(Debug)]
struct MessageA {
    id: u8,
}

#[derive(Debug)]
struct MessageB {
    id: u8,
}

impl Listener<MessageA> for Handler {
    fn handle(&self, event: &Event<MessageA>) {
        info!("message a event: {:?}", event);
        assert_ne!(event.deref().id, 2);
    }
}

impl Listener<MessageB> for Handler {
    fn handle(&self, event: &Event<MessageB>) {
        info!("message b event: {:?}", event);
    }
}

fn main() {
    pretty_env_logger::init();
    let eventbus = Eventbus::new();

    // create a topic
    let topic = TopicKey::from("foobar");
    // register listener for type `MessageA`
    let handler_a = eventbus.register::<MessageA, _, _>(topic.clone(), Handler);
    // register listener for type `MessageB`
    let handler_b = eventbus.register::<MessageB, _, _>(topic.clone(), Handler);

    // post sample
    // get `MessageA` `Topic`
    let topic_message_a = eventbus.create_topic(topic.clone());
    let event_a = Event::new(topic_message_a.get_key().clone(), MessageA { id: 1 });
    // shorthand post via `Topic`
    topic_message_a.post(&event_a);
    // create `MessageB` `Event`
    let event_b = Event::new(topic.clone(), MessageB { id: 1 });
    // can also post via eventbus
    eventbus.post(&event_b);

    // unregister sample
    // this should not produce any output since we already unregister listener
    handler_a.unregister();
    let event = Event::new(topic_message_a.get_key().clone(), MessageA { id: 2 });
    topic_message_a.post(&event);
}

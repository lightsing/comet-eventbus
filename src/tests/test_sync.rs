use crate::*;

struct Handler;

#[derive(Debug)]
struct Message {
    id: u8,
}

impl Listener<Message> for Handler {
    fn handle(&self, event: &Event<Message>) {
        println!("{:?}", event)
    }
}

#[test]
fn test() {
    let eventbus = Eventbus::new();
    let topic = TopicKey::from("foobar");
    let handler = EventListener::new(topic.clone(), Handler);
    eventbus.register(handler);
    let topic = eventbus.create_topic(topic.clone());
    let event = Event::new(topic.key.clone(), Message { id: 1 });
    topic.post(&event);
}
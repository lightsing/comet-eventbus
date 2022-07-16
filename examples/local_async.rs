use comet_eventbus::*;
use log::info;
use std::ops::Deref;

struct Handler;

#[derive(Debug)]
struct Message {
    id: u8,
}

#[comet_eventbus::async_trait]
impl Listener<Message> for Handler {
    async fn handle(&self, event: &Event<Message>) {
        info!("A: {:?}", event);
        assert_ne!(event.deref().id, 2);
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let eventbus = Eventbus::new();
    let topic = TopicKey::from("foobar");
    let handler = eventbus.register(topic.clone(), Handler).await;
    let topic = eventbus.create_topic(topic.clone()).await;
    let event = Event::new(topic.get_key().clone(), Message { id: 1 });
    topic.post(&event).await;
    // this should not produce any output since we already unregister listener
    handler.unregister().await;
    let event = Event::new(topic.get_key().clone(), Message { id: 2 });
    topic.post(&event).await;
}

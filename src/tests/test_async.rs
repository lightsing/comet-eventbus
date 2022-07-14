use crate::*;

struct Handler;

#[derive(Debug)]
struct Message {
    id: u8,
}

#[async_trait::async_trait]
impl Listener<Message> for Handler {
    async fn handle(&self, event: &Event<Message>) {
        println!("{:?}", event)
    }
}

#[tokio::test]
async fn test() {
    let eventbus = Eventbus::new();
    let topic = TopicKey::from("foobar");
    let handler = eventbus.register(topic.clone(), Handler).await;
    let topic = eventbus.create_topic(topic.clone()).await;
    let event = Event::new(topic.key.clone(), Message { id: 1});
    topic.post(&event).await;
    handler.unregister().await;
    let event = Event::new(topic.key.clone(), Message { id: 2 });
    // this should not produce any output since we already unregister listener
    topic.post(&event).await;
}
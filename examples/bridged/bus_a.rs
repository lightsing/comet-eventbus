use comet_eventbus::bridge::*;
use comet_eventbus::*;

use crate::def::Message;

struct HandlerA;

#[comet_eventbus::async_trait]
impl Listener<Message> for HandlerA {
    async fn handle(&self, event: &Event<Message>) {
        println!("A: {:?}", event)
    }
}

pub async fn main() {
    let eventbus_a = Eventbus::new();
    let bridged_a = EventbusBridge::new(eventbus_a);

    let server_a = bridged_a.clone().listen("127.0.0.1:50001".parse().unwrap());
    tokio::spawn(server_a);

    bridged_a.connect("http://127.0.0.1:50002").await.unwrap();

    let topic = TopicKey::from("foobar");

    let handler_a = bridged_a.register(topic.clone(), HandlerA).await;
    let topic_a = bridged_a.create_topic(topic.clone()).await;
    let event = Event::new(topic_a.get_key().clone(), Message { id: 1 });
    topic_a.post(&event).await.unwrap();
    handler_a.unregister().await;
}

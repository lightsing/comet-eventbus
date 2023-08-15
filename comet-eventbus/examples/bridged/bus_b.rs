use comet_eventbus::bridge::*;
use comet_eventbus::*;

use crate::def::Message;

struct HandlerB;

#[async_trait::async_trait]
impl Listener<Message> for HandlerB {
    async fn handle(&self, event: &Event<Message>) -> Result<(), ListenerError> {
        println!("B: {:?}", event);
        Ok(())
    }
}

pub async fn main() {
    let eventbus_b = Eventbus::new();
    let bridged_b = EventbusBridge::new(eventbus_b);

    tokio::spawn(bridged_b.clone().listen("127.0.0.1:50002".parse().unwrap()));
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    bridged_b.connect("http://127.0.0.1:50001").await.unwrap();

    let topic = TopicKey::from("foobar");

    let handler_b = bridged_b.register(topic.clone(), HandlerB).await;
    let topic_b = bridged_b.create_topic(topic.clone()).await;
    let event = Event::new(topic_b.get_key().clone(), Message { id: 2 });
    topic_b.post(&event).await.unwrap();
    handler_b.unregister().await;
}

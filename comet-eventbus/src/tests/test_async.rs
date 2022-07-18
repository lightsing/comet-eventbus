use std::time::Duration;
use crate::bridge::EventbusBridge;
use crate::*;
use serde::{Deserialize, Serialize};

struct HandlerA;
struct HandlerB;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: u8,
}

#[async_trait::async_trait]
impl Listener<Message> for HandlerA {
    async fn handle(&self, event: &Event<Message>) {
        println!("A: {:?}", event)
    }
}
#[async_trait::async_trait]
impl Listener<Message> for HandlerB {
    async fn handle(&self, event: &Event<Message>) {
        println!("B: {:?}", event)
    }
}

#[tokio::test]
async fn test() {
    let eventbus = Eventbus::new();
    let topic = TopicKey::from("foobar");
    let handler = eventbus.register(topic.clone(), HandlerA).await;
    let topic = eventbus.create_topic(topic.clone()).await;
    let event = Event::new(topic.key.clone(), Message { id: 1 });
    topic.post(&event).await;
    handler.unregister().await;
    let event = Event::new(topic.key.clone(), Message { id: 2 });
    // this should not produce any output since we already unregister listener
    topic.post(&event).await;
}

#[tokio::test]
async fn test_bridge() {
    let eventbus_a = Eventbus::new();
    let eventbus_b = Eventbus::new();
    let bridged_a = EventbusBridge::new(eventbus_a);
    let bridged_b = EventbusBridge::new(eventbus_b);

    let server_a = bridged_a.clone().listen("127.0.0.1:50001".parse().unwrap());
    tokio::spawn(server_a);
    let server_b = bridged_b.clone().listen("127.0.0.1:50002".parse().unwrap());
    tokio::spawn(server_b);

    tokio::time::sleep(Duration::from_secs(5)).await;
    bridged_a.connect("http://127.0.0.1:50002").await.unwrap();
    bridged_b.connect("http://127.0.0.1:50001").await.unwrap();

    let topic = TopicKey::from("foobar");

    let handler_a = bridged_a.register(topic.clone(), HandlerA).await;
    let handler_b = bridged_b.register(topic.clone(), HandlerB).await;

    let topic_a = bridged_a.create_topic(topic.clone()).await;
    let event = Event::new(topic_a.get_key().clone(), Message { id: 1 });
    topic_a.post(&event).await.unwrap();

    let topic_b = bridged_b.create_topic(topic.clone()).await;
    let event = Event::new(topic_b.get_key().clone(), Message { id: 2 });
    topic_b.post(&event).await.unwrap();

    handler_a.unregister().await;
    let topic_b = bridged_b.create_topic(topic.clone()).await;
    let event = Event::new(topic_b.get_key().clone(), Message { id: 2 });
    topic_b.post(&event).await.unwrap();

    handler_b.unregister().await;
    let topic_a = bridged_a.create_topic(topic.clone()).await;
    let event = Event::new(topic_a.get_key().clone(), Message { id: 1 });
    topic_a.post(&event).await.unwrap();
}

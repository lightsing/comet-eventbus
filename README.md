# comet-eventbus

[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)](
https://github.com/lightsing/comet-eventbus#license)
[![Documentation](https://img.shields.io/badge/docs-latest-green)](
https://lightsing.github.io/comet-eventbus/comet_eventbus/index.html)

A strong typed sync and asynchronous eventbus implementation.

Also provide grpc eventbus bridge for asynchronous implementation.

### Notice: This crate is under highly active development. I won't publish it on crates.io, before the api becomes stable.

## Usage

### Async Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
comet-eventbus = { git = "https://github.com/lightsing/comet-eventbus.git" }
```

### Sync Usage
Add this to your `Cargo.toml`:
```toml
[dependencies.comet-eventbus]
git = "https://github.com/lightsing/comet-eventbus.git"
features = ["sync", "sync_parallel"]
default-features = false
```

## Example

### Local Usage

```rust
use comet_eventbus::*;

struct Handler;

#[derive(Debug)]
struct Message {
    id: u8,
}

#[comet_eventbus::async_trait]
impl Listener<Message> for Handler {
    async fn handle(&self, event: &Event<Message>) {
        println!("{:?}", event)
    }
}

#[tokio::main]
async fn main() {
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
```

#### Console Output
```
Event<Message> { topic: TopicKey("foobar"), message: Message { id: 1 } }
```

### Bridge Usage

This example creates two eventbus and connect them together.

#### Common Definition
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: u8,
}
```

#### Eventbus A
```rust
use comet_eventbus::*;
use comet_eventbus::bridge::*;

struct HandlerA;

#[async_trait::async_trait]
impl Listener<Message> for HandlerA {
    async fn handle(&self, event: &Event<Message>) {
        println!("A: {:?}", event)
    }
}

#[tokio::main]
async fn main() {
    let eventbus_a = Eventbus::new();
    let bridged_a = EventbusBridge::new(eventbus_a);

    let server_a = bridged_a.clone().listen("127.0.0.1:50001".parse().unwrap());
    tokio::spawn(server_a);

    bridged_a.connect("http://127.0.0.1:50002").await.unwrap();

    let topic = TopicKey::from("foobar");

    let handler_a = bridged_a.register(topic.clone(), HandlerA).await;

    let topic_a = bridged_a.create_topic(topic.clone()).await;
    let event = Event::new(topic_a.get_key().clone(), Message { id: 1 });
    topic_a.post(&event).await;
}
```

#### Eventbus B
```rust
use comet_eventbus::*;
use comet_eventbus::bridge::*;

struct HandlerB;

#[async_trait::async_trait]
impl Listener<Message> for HandlerB {
    async fn handle(&self, event: &Event<Message>) {
        println!("B: {:?}", event)
    }
}

#[tokio::main]
async fn main() {
    let eventbus_b = Eventbus::new();
    let bridged_b = EventbusBridge::new(eventbus_b);

    let server_b = bridged_b.clone().listen("127.0.0.1:50002".parse().unwrap());
    tokio::spawn(server_b);

    bridged_b.connect("http://127.0.0.1:50001").await.unwrap();

    let topic = TopicKey::from("foobar");

    let handler_b = bridged_b.register(topic.clone(), HandlerB).await;

    let topic_b = bridged_b.create_topic(topic.clone()).await;
    let event = Event::new(topic_b.get_key().clone(), Message { id: 2 });
    topic_b.post(&event).await;
}
```

#### Console Output
```
B: Event<comet_eventbus::tests::test_async::Message> { topic: TopicKey("foobar"), message: Message { id: 1 } }
A: Event<comet_eventbus::tests::test_async::Message> { topic: TopicKey("foobar"), message: Message { id: 2 } }
```
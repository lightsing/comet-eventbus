# comet-eventbus

[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)](
https://github.com/lightsing/comet-eventbus#license)
[![Documentation](https://img.shields.io/badge/docs-latest-green)](
https://lightsing.github.io/comet-eventbus/comet_eventbus/index.html)

A strong typed sync and asynchronous eventbus implementation.

## Usage

### async usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
async-trait = "0.1"
comet-eventbus = { git = "https://github.com/lightsing/comet-eventbus.git" }
```

### sync usage
Add this to your `Cargo.toml`:
```toml
[dependencies.comet-eventbus]
git = "https://github.com/lightsing/comet-eventbus.git"
features = ["sync", "sync_parallel"]
default-features = false
```

## Example

```rust
use comet_eventbus::*;

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

async fn test() {
    let eventbus = Eventbus::new();
    let topic = TopicKey::from("foobar");
    let handler = EventListener::new(topic.clone(), Handler);
    eventbus.register(handler).await;
    let topic = eventbus.create_topic(topic.clone()).await;
    let event = Event::new(topic.key.clone(), Message { id: 1});
    topic.post(&event).await;
}
```

Console Output:
```
Event<Message> { topic: TopicKey("foobar"), message: Message { id: 1 } }
```
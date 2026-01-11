use lavalink_rs::{hook, model::events, prelude::*};

pub fn raw_event<'a>(_: LavalinkClient, session_id: String, event: &'a serde_json::Value) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
    Box::pin(async move {
        if event["op"].as_str() == Some("event") || event["op"].as_str() == Some("playerUpdate") {
            info!("{:?} -> {:?}", session_id, event);
        }
    })
}

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    info!("{:?} -> {:?}", session_id, event);
}

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use lavalink_rs::model::track::TrackData;
use lavalink_rs::prelude::{SearchEngines, TrackInQueue, TrackLoadData};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

use crate::Data;

pub type WsStream = WebSocketStream<TcpStream>;
pub type WsSender = SplitSink<WsStream, Message>;
pub type WsReceiver = SplitStream<WsStream>;
pub type ClientConnections = Arc<Mutex<HashMap<u64, Vec<Arc<RwLock<WsSender>>>>>>;

pub struct WebSocketServer {
    addr: SocketAddr,
    data: Arc<Data>,
    clients: ClientConnections,
}

impl WebSocketServer {
    pub fn get_clients(&self) -> ClientConnections {
        Arc::clone(&self.clients)
    }
}

impl WebSocketServer {
    pub fn new(addr: SocketAddr, data: Arc<Data>) -> Self {
        Self {
            addr,
            data,
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(self.addr).await?;
        info!("WebSocket server listening on: {}", self.addr);

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    info!("New WebSocket connection from: {}", peer_addr);

                    let data = Arc::clone(&self.data);
                    let clients = Arc::clone(&self.clients);
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, peer_addr, data, clients).await {
                            error!("Error handling connection from {}: {}", peer_addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    peer_addr: std::net::SocketAddr,
    data: Arc<Data>,
    clients: ClientConnections,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = tokio_tungstenite::accept_async(stream).await?;
    info!("WebSocket connection established with: {}", peer_addr);

    let (sender, receiver) = ws_stream.split();
    let sender = Arc::new(RwLock::new(sender));

    let sender_clone = Arc::clone(&sender);
    let data_clone = Arc::clone(&data);
    let clients_clone = Arc::clone(&clients);

    match tokio::spawn(async move {
        handle_client_messages(receiver, sender_clone, data_clone, clients_clone).await
    })
    .await
    {
        Ok(Ok(())) => {
            info!("WebSocket connection closed normally: {}", peer_addr);
        }
        Ok(Err(e)) => {
            error!("WebSocket handler error from {}: {}", peer_addr, e);
        }
        Err(e) => {
            error!("Task join error from {}: {}", peer_addr, e);
        }
    }

    Ok(())
}

async fn handle_client_messages(
    mut receiver: WsReceiver,
    sender: Arc<RwLock<WsSender>>,
    data: Arc<Data>,
    clients: ClientConnections,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut subscribed_guild: Option<u64> = None;

    while let Some(msg) = receiver.next().await {
        match msg? {
            Message::Text(text) => {
                info!("Received message from client: {}", text);

                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(payload) => {
                        let msg_type = payload
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        info!("Processing message type: {}", msg_type);

                        if let Some("subscribe") = payload.get("type").and_then(|v| v.as_str()) {
                            if let Some(guild_id_str) =
                                payload.get("guild_id").and_then(|v| v.as_str())
                            {
                                if let Ok(guild_id) = guild_id_str.parse::<u64>() {
                                    subscribed_guild = Some(guild_id);
                                    let mut client_map = clients.lock().await;
                                    client_map
                                        .entry(guild_id)
                                        .or_insert_with(Vec::new)
                                        .push(Arc::clone(&sender));
                                    info!("Client subscribed to guild: {}", guild_id);
                                    let _ = send_response(
                                        &sender,
                                        "subscribed",
                                        Some(serde_json::json!({
                                            "guild_id": guild_id_str
                                        })),
                                    )
                                    .await;
                                    continue;
                                }
                            }
                        }

                        if let Err(e) = process_message(&payload, &sender, &data).await {
                            error!("Error processing message: {}", e);
                            let _ = send_error_response(
                                &sender,
                                format!("Error processing message: {}", e),
                            )
                            .await;
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse message: {}", e);
                        let _ = send_error_response(&sender, format!("Invalid JSON format: {}", e))
                            .await;
                    }
                }
            }
            Message::Binary(data) => {
                debug!("Received binary message from client: {} bytes", data.len());
            }
            Message::Ping(ping) => {
                if let Ok(_) = sender.write().await.send(Message::Pong(ping)).await {};
            }
            Message::Pong(_) => {
                debug!("Received pong from client");
            }
            Message::Close(_) => {
                info!("Client initiated close");

                if let Some(guild_id) = subscribed_guild {
                    let mut client_map = clients.lock().await;
                    client_map.remove(&guild_id);
                }
                break;
            }
            Message::Frame(_) => {
                debug!("Received frame message");
            }
        }
    }

    Ok(())
}

async fn process_message(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let message_type = payload
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'type' field")?;

    match message_type {
        "ping" => {
            send_response(sender, "pong", None).await?;
        }
        "status" => {
            let status = serde_json::json!({
                "status": "connected",
                "lavalink_connected": true,
                "version": env!("CARGO_PKG_VERSION")
            });
            send_response(sender, "status", Some(status)).await?;
        }
        "player_info" => {
            handle_player_info(payload, sender, data).await?;
        }
        "play" => {
            handle_play_request(payload, sender, data).await?;
        }
        "skip" => {
            handle_skip_request(payload, sender, data).await?;
        }
        "pause" => {
            handle_pause_request(payload, sender, data).await?;
        }
        "resume" => {
            handle_resume_request(payload, sender, data).await?;
        }
        "stop" => {
            handle_stop_request(payload, sender, data).await?;
        }
        "volume" => {
            handle_volume_request(payload, sender, data).await?;
        }
        "seek" => {
            handle_seek_request(payload, sender, data).await?;
        }
        "queue" => {
            handle_queue_request(payload, sender, data).await?;
        }
        _ => {
            error!("Unknown message type: {}", message_type);
            send_error_response(sender, format!("Unknown message type: {}", message_type)).await?;
        }
    }

    Ok(())
}

async fn handle_play_request(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("[PLAY] Starting play request handler");

    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let track_id = payload
        .get("track_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'track_id' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    info!(
        "[PLAY] Request for guild: {}, track: {}",
        guild_id, track_id
    );

    if let Some(player) = data.lavalink.get_player_context(guild_id) {
        let query = if track_id.starts_with("http") {
            track_id
        } else {
            &SearchEngines::Spotify.to_query(&track_id)?
        };

        let loaded_tracks = data.lavalink.load_tracks(guild_id, &query).await?;

        let mut tracks: Vec<TrackInQueue> = match loaded_tracks.data {
            Some(TrackLoadData::Track(x)) => vec![x.into()],
            Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
            Some(TrackLoadData::Playlist(x)) => x.tracks.iter().map(|x| x.clone().into()).collect(),
            _ => {
                return Ok(());
            }
        };
        let queue = player.get_queue();
        queue.append(tracks.into())?;

        if player.get_player().await?.track.is_none() {
            player.skip()?;
        }
        if let Ok(player_data) = player.get_player().await
            && player_data.track.is_none()
            && queue.get_track(0).await.is_ok_and(|x| x.is_some())
        {
            player.skip()?;
        }

        info!("[PLAY] Player context found for guild {}", guild_id);
    } else {
        info!("[PLAY] No player context found for guild {}", guild_id);
    }

    let response = serde_json::json!({
        "status": "playing",
        "guild_id": guild_id_str,
        "track_id": track_id,
        "message": "Play command sent to Lavalink"
    });

    info!("[PLAY] About to send response");
    send_response(sender, "play_response", Some(response)).await?;
    info!("[PLAY] Response sent successfully");
    Ok(())
}

async fn handle_skip_request(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    info!("Skip request for guild: {}", guild_id);

    if let Some(player) = data.lavalink.get_player_context(guild_id) {
        if let Err(e) = player.skip() {
            error!("Failed to skip track: {}", e);
            send_error_response(sender, format!("Failed to skip: {}", e)).await?;
            return Ok(());
        }
    }

    let response = serde_json::json!({
        "status": "skipped",
        "guild_id": guild_id_str
    });

    send_response(sender, "skip_response", Some(response)).await?;
    Ok(())
}

async fn handle_pause_request(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    info!("Pause request for guild: {}", guild_id);

    if let Some(player) = data.lavalink.get_player_context(guild_id) {
        if let Err(e) = player.set_pause(true).await {
            error!("Failed to pause: {}", e);
            send_error_response(sender, format!("Failed to pause: {}", e)).await?;
            return Ok(());
        }
    }

    let response = serde_json::json!({
        "status": "paused",
        "guild_id": guild_id_str
    });

    send_response(sender, "pause_response", Some(response)).await?;
    Ok(())
}

async fn handle_resume_request(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    info!("Resume request for guild: {}", guild_id);

    if let Some(player) = data.lavalink.get_player_context(guild_id) {
        if let Err(e) = player.set_pause(false).await {
            error!("Failed to resume: {}", e);
            send_error_response(sender, format!("Failed to resume: {}", e)).await?;
            return Ok(());
        }
    }

    let response = serde_json::json!({
        "status": "resumed",
        "guild_id": guild_id_str
    });

    send_response(sender, "resume_response", Some(response)).await?;
    Ok(())
}

async fn handle_stop_request(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    info!("Stop request for guild: {}", guild_id);

    if let Some(player) = data.lavalink.get_player_context(guild_id) {
        if let Err(e) = player.stop_now().await {
            error!("Failed to stop: {}", e);
            send_error_response(sender, format!("Failed to stop: {}", e)).await?;
            return Ok(());
        }
    }

    let response = serde_json::json!({
        "status": "stopped",
        "guild_id": guild_id_str
    });

    send_response(sender, "stop_response", Some(response)).await?;
    Ok(())
}

async fn handle_volume_request(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let volume = payload
        .get("volume")
        .and_then(|v| v.as_i64())
        .ok_or("Missing or invalid 'volume' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    if volume < 0 || volume > 1000 {
        return Err("Volume must be between 0 and 1000".into());
    }

    info!("Volume request for guild: {}, volume: {}", guild_id, volume);

    if let Some(player) = data.lavalink.get_player_context(guild_id) {
        if let Err(e) = player.set_volume(volume as u16).await {
            error!("Failed to set volume: {}", e);
            send_error_response(sender, format!("Failed to set volume: {}", e)).await?;
            return Ok(());
        }
    }

    let response = serde_json::json!({
        "status": "volume_changed",
        "guild_id": guild_id_str,
        "volume": volume
    });

    send_response(sender, "volume_response", Some(response)).await?;
    Ok(())
}

async fn handle_queue_request(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    info!("Queue request for guild: {}", guild_id);

    let mut queue: Vec<TrackData> = Vec::new();
    let mut current_track: Option<TrackData> = None;

    if let Some(player) = data.lavalink.get_player_context(guild_id) {
        let player_data = player.get_player().await?;
        let queue_data = player.get_queue();
        let current = player_data.track;
        let queue_count = queue_data.get_count().await?;
        for idx in 0..queue_count {
            if let Ok(Some(track)) = queue_data.get_track(idx).await {
                queue.push(track.track);
            }
        }
        current_track = current;
    }

    let response = if let Some(track) = current_track {
        serde_json::json!({
            "status": "queue_info",
            "guild_id": guild_id_str,
            "current_track": track,
            "queue": queue
        })
    } else {
        serde_json::json!({
            "status": "queue_info",
            "guild_id": guild_id_str,
            "current_track": serde_json::json!(null),
            "queue": queue
        })
    };

    send_response(sender, "queue_response", Some(response)).await?;
    Ok(())
}

async fn handle_seek_request(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let position = payload
        .get("position")
        .and_then(|v| v.as_i64())
        .ok_or("Missing or invalid 'position' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    if position < 0 {
        return Err("Position must be a positive value".into());
    }

    info!(
        "Seek request for guild: {}, position: {}",
        guild_id, position
    );

    if let Some(_player) = data.lavalink.get_player_context(guild_id) {
        info!("Seek position set to: {} ms", position);
    }

    let response = serde_json::json!({
        "status": "seeked",
        "guild_id": guild_id_str,
        "position": position
    });

    send_response(sender, "seek_response", Some(response)).await?;
    Ok(())
}

async fn handle_player_info(
    payload: &serde_json::Value,
    sender: &Arc<RwLock<WsSender>>,
    data: &Arc<Data>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id_str = payload
        .get("guild_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'guild_id' field")?;

    let guild_id: u64 = guild_id_str
        .parse()
        .map_err(|_| "Invalid guild_id format")?;

    info!("Player info request for guild: {}", guild_id);

    let mut player_info = serde_json::json!({
        "guild_id": guild_id_str,
        "is_paused": true,
        "position": 0i64,
        "volume": 100,
        "current_track": serde_json::json!(null)
    });

    if let Some(player) = data.lavalink.get_player_context(guild_id) {
        let player_ctx = player.get_player().await?;
        let position = player_ctx.state.position;
        player_info["is_paused"] = serde_json::Value::Bool(player_ctx.paused);
        player_info["position"] = serde_json::Value::Number(position.into());
        player_info["status"] = serde_json::Value::String("connected".to_string());
    }

    send_response(sender, "player_info_response", Some(player_info)).await?;
    Ok(())
}

async fn send_response(
    sender: &Arc<RwLock<WsSender>>,
    response_type: &str,
    data: Option<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut response = serde_json::json!({
        "type": response_type,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    if let Some(data) = data {
        response
            .as_object_mut()
            .unwrap()
            .extend(data.as_object().unwrap().clone());
    }

    let response_str = response.to_string();
    info!("Sending response: {}", response_str);

    let mut sender_guard = sender.write().await;
    match sender_guard.send(Message::Text(response_str.into())).await {
        Ok(_) => {
            info!("Response sent successfully for type: {}", response_type);
            Ok(())
        }
        Err(e) => {
            error!("Failed to send response: {}", e);
            Err(e.into())
        }
    }
}

async fn send_error_response(
    sender: &Arc<RwLock<WsSender>>,
    error: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = serde_json::json!({
        "type": "error",
        "error": error,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let mut sender_guard = sender.write().await;
    sender_guard
        .send(Message::Text(response.to_string().into()))
        .await?;

    Ok(())
}

pub async fn broadcast_track_update(
    clients: &ClientConnections,
    guild_id: u64,
    track_info: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client_map = clients.lock().await;

    if let Some(senders) = client_map.get(&guild_id) {
        let event = serde_json::json!({
            "type": "trackUpdate",
            "guild_id": guild_id.to_string(),
            "track": track_info,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let message_text = event.to_string();

        for sender in senders {
            if let Err(e) = sender
                .write()
                .await
                .send(Message::Text(message_text.clone().into()))
                .await
            {
                error!("Failed to send trackUpdate to client: {}", e);
            }
        }
    }

    Ok(())
}

pub async fn broadcast_player_event(
    clients: &ClientConnections,
    guild_id: u64,
    event_type: &str,
    event_data: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client_map = clients.lock().await;

    if let Some(senders) = client_map.get(&guild_id) {
        let event = serde_json::json!({
            "type": "playerEvent",
            "event_type": event_type,
            "guild_id": guild_id.to_string(),
            "data": event_data,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let message_text = event.to_string();

        for sender in senders {
            if let Err(e) = sender
                .write()
                .await
                .send(Message::Text(message_text.clone().into()))
                .await
            {
                error!("Failed to send playerEvent to client: {}", e);
            }
        }
    }

    Ok(())
}

use crate::websocket::server::ClientConnections;
use poise::serenity_prelude as serenity;
use std::sync::Arc;

#[derive(Clone)]
pub struct PlayerData {
    pub channel_id: serenity::ChannelId,
    pub http: Arc<serenity::Http>,
    pub db: sqlx::SqlitePool,
    pub ws_clients: Option<ClientConnections>,
}

impl PlayerData {
    pub const fn new(
        channel_id: serenity::ChannelId,
        http: Arc<serenity::Http>,
        db: sqlx::SqlitePool,
    ) -> Self {
        Self {
            channel_id,
            http,
            db,
            ws_clients: None,
        }
    }

    pub fn with_ws_clients(
        channel_id: serenity::ChannelId,
        http: Arc<serenity::Http>,
        db: sqlx::SqlitePool,
        ws_clients: ClientConnections,
    ) -> Self {
        Self {
            channel_id,
            http,
            db,
            ws_clients: Some(ws_clients),
        }
    }
}

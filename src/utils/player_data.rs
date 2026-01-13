use poise::serenity_prelude as serenity;
use std::sync::Arc;

#[derive(Clone)]
pub struct PlayerData {
    pub channel_id: serenity::ChannelId,
    pub http: Arc<serenity::Http>,
    pub db: sqlx::SqlitePool,
}

impl PlayerData {
    pub fn new(
        channel_id: serenity::ChannelId,
        http: Arc<serenity::Http>,
        db: sqlx::SqlitePool,
    ) -> Self {
        Self {
            channel_id,
            http,
            db,
        }
    }
}

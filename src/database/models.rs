use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GuildConfig {
    pub guild_id: i64,
    pub dj_role_id: Option<i64>,
    pub volume: i16,
    pub auto_disconnect: bool,
    pub auto_disconnect_time: i32,
    pub announce_songs: bool,
    pub announce_channel_id: Option<i64>,
    pub max_queue_length: i32,
    pub allow_filters: bool,
    pub allow_explicit: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self {
            guild_id: 0,
            dj_role_id: None,
            volume: 100,
            auto_disconnect: true,
            auto_disconnect_time: 300,
            announce_songs: true,
            announce_channel_id: None,
            max_queue_length: 100,
            allow_filters: true,
            allow_explicit: true,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserFavorite {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub author: String,
    pub uri: String,
    pub artwork_url: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GuildPlaylist {
    pub id: i64,
    pub guild_id: i64,
    pub name: String,
    pub created_by: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PlaylistTrack {
    pub id: i64,
    pub playlist_id: i64,
    pub title: String,
    pub author: String,
    pub uri: String,
    pub position: i32,
    pub added_by: i64,
    pub added_at: String,
}

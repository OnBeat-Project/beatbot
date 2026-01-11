use super::models::*;
use sqlx::{Result, SqlitePool};

pub async fn get_guild_config(pool: &SqlitePool, guild_id: i64) -> Result<GuildConfig> {
    match sqlx::query_as::<_, GuildConfig>("SELECT * FROM guild_configs WHERE guild_id = ?")
        .bind(guild_id)
        .fetch_optional(pool)
        .await?
    {
        Some(config) => Ok(config),
        None => create_guild_config(pool, guild_id).await,
    }
}

pub async fn create_guild_config(pool: &SqlitePool, guild_id: i64) -> Result<GuildConfig> {
    sqlx::query_as::<_, GuildConfig>(
        "INSERT INTO guild_configs (guild_id) 
         VALUES (?)
         RETURNING *",
    )
    .bind(guild_id)
    .fetch_one(pool)
    .await
}

pub async fn update_dj_role(pool: &SqlitePool, guild_id: i64, role_id: Option<i64>) -> Result<()> {
    sqlx::query(
        "UPDATE guild_configs 
         SET dj_role_id = ?, updated_at = CURRENT_TIMESTAMP 
         WHERE guild_id = ?",
    )
    .bind(role_id)
    .bind(guild_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_volume(pool: &SqlitePool, guild_id: i64, volume: i16) -> Result<()> {
    sqlx::query(
        "UPDATE guild_configs 
         SET volume = ?, updated_at = CURRENT_TIMESTAMP 
         WHERE guild_id = ?",
    )
    .bind(volume)
    .bind(guild_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_auto_disconnect(
    pool: &SqlitePool,
    guild_id: i64,
    enabled: bool,
    time: Option<i32>,
) -> Result<()> {
    let query = if let Some(time) = time {
        sqlx::query(
            "UPDATE guild_configs 
             SET auto_disconnect = ?, auto_disconnect_time = ?, updated_at = CURRENT_TIMESTAMP 
             WHERE guild_id = ?",
        )
        .bind(enabled)
        .bind(time)
        .bind(guild_id)
    } else {
        sqlx::query(
            "UPDATE guild_configs 
             SET auto_disconnect = ?, updated_at = CURRENT_TIMESTAMP 
             WHERE guild_id = ?",
        )
        .bind(enabled)
        .bind(guild_id)
    };

    query.execute(pool).await?;
    Ok(())
}

pub async fn update_announce_settings(
    pool: &SqlitePool,
    guild_id: i64,
    announce_songs: bool,
    channel_id: Option<i64>,
) -> Result<()> {
    sqlx::query(
        "UPDATE guild_configs 
         SET announce_songs = ?, announce_channel_id = ?, updated_at = CURRENT_TIMESTAMP 
         WHERE guild_id = ?",
    )
    .bind(announce_songs)
    .bind(channel_id)
    .bind(guild_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_max_queue_length(
    pool: &SqlitePool,
    guild_id: i64,
    max_length: i32,
) -> Result<()> {
    sqlx::query(
        "UPDATE guild_configs 
         SET max_queue_length = ?, updated_at = CURRENT_TIMESTAMP 
         WHERE guild_id = ?",
    )
    .bind(max_length)
    .bind(guild_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_filters_setting(
    pool: &SqlitePool,
    guild_id: i64,
    allow_filters: bool,
) -> Result<()> {
    sqlx::query(
        "UPDATE guild_configs 
         SET allow_filters = ?, updated_at = CURRENT_TIMESTAMP 
         WHERE guild_id = ?",
    )
    .bind(allow_filters)
    .bind(guild_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn reset_guild_config(pool: &SqlitePool, guild_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM guild_configs WHERE guild_id = ?")
        .bind(guild_id)
        .execute(pool)
        .await?;

    create_guild_config(pool, guild_id).await?;
    Ok(())
}

pub async fn get_user_favorites(pool: &SqlitePool, user_id: i64) -> Result<Vec<UserFavorite>> {
    sqlx::query_as::<_, UserFavorite>(
        "SELECT * FROM user_favorites WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn add_favorite(
    pool: &SqlitePool,
    user_id: i64,
    title: &str,
    author: &str,
    uri: &str,
    artwork_url: Option<&str>,
) -> Result<UserFavorite> {
    sqlx::query_as::<_, UserFavorite>(
        "INSERT INTO user_favorites (user_id, title, author, uri, artwork_url)
         VALUES (?, ?, ?, ?, ?)
         RETURNING *",
    )
    .bind(user_id)
    .bind(title)
    .bind(author)
    .bind(uri)
    .bind(artwork_url)
    .fetch_one(pool)
    .await
}

pub async fn remove_favorite(pool: &SqlitePool, user_id: i64, favorite_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM user_favorites WHERE id = ? AND user_id = ?")
        .bind(favorite_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn check_favorite_exists(pool: &SqlitePool, user_id: i64, uri: &str) -> Result<bool> {
    let result: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM user_favorites WHERE user_id = ? AND uri = ?")
            .bind(user_id)
            .bind(uri)
            .fetch_optional(pool)
            .await?;

    Ok(result.is_some())
}

pub async fn get_guild_playlists(pool: &SqlitePool, guild_id: i64) -> Result<Vec<GuildPlaylist>> {
    sqlx::query_as::<_, GuildPlaylist>(
        "SELECT * FROM guild_playlists WHERE guild_id = ? ORDER BY created_at DESC",
    )
    .bind(guild_id)
    .fetch_all(pool)
    .await
}

pub async fn create_playlist(
    pool: &SqlitePool,
    guild_id: i64,
    name: &str,
    created_by: i64,
) -> Result<GuildPlaylist> {
    sqlx::query_as::<_, GuildPlaylist>(
        "INSERT INTO guild_playlists (guild_id, name, created_by)
         VALUES (?, ?, ?)
         RETURNING *",
    )
    .bind(guild_id)
    .bind(name)
    .bind(created_by)
    .fetch_one(pool)
    .await
}

pub async fn delete_playlist(pool: &SqlitePool, playlist_id: i64, guild_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM playlist_tracks WHERE playlist_id = ?")
        .bind(playlist_id)
        .execute(pool)
        .await?;

    sqlx::query("DELETE FROM guild_playlists WHERE id = ? AND guild_id = ?")
        .bind(playlist_id)
        .bind(guild_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_playlist_tracks(
    pool: &SqlitePool,
    playlist_id: i64,
) -> Result<Vec<PlaylistTrack>> {
    sqlx::query_as::<_, PlaylistTrack>(
        "SELECT * FROM playlist_tracks WHERE playlist_id = ? ORDER BY position",
    )
    .bind(playlist_id)
    .fetch_all(pool)
    .await
}

pub async fn add_track_to_playlist(
    pool: &SqlitePool,
    playlist_id: i64,
    title: &str,
    author: &str,
    uri: &str,
    added_by: i64,
) -> Result<PlaylistTrack> {
    let position: (i32,) = sqlx::query_as(
        "SELECT COALESCE(MAX(position), -1) + 1 FROM playlist_tracks WHERE playlist_id = ?",
    )
    .bind(playlist_id)
    .fetch_one(pool)
    .await?;

    sqlx::query_as::<_, PlaylistTrack>(
        "INSERT INTO playlist_tracks (playlist_id, title, author, uri, position, added_by)
         VALUES (?, ?, ?, ?, ?, ?)
         RETURNING *",
    )
    .bind(playlist_id)
    .bind(title)
    .bind(author)
    .bind(uri)
    .bind(position.0)
    .bind(added_by)
    .fetch_one(pool)
    .await
}

pub async fn remove_track_from_playlist(
    pool: &SqlitePool,
    track_id: i64,
    playlist_id: i64,
) -> Result<()> {
    sqlx::query("DELETE FROM playlist_tracks WHERE id = ? AND playlist_id = ?")
        .bind(track_id)
        .bind(playlist_id)
        .execute(pool)
        .await?;
    Ok(())
}

-- Add migration script here

CREATE TABLE IF NOT EXISTS guild_configs (
    guild_id INTEGER PRIMARY KEY,
    dj_role_id INTEGER,
    volume INTEGER DEFAULT 100 CHECK(volume >= 0 AND volume <= 200),
    auto_disconnect BOOLEAN DEFAULT 1,
    auto_disconnect_time INTEGER DEFAULT 300,
    announce_songs BOOLEAN DEFAULT 1,
    announce_channel_id INTEGER,
    max_queue_length INTEGER DEFAULT 100,
    allow_filters BOOLEAN DEFAULT 1,
    allow_explicit BOOLEAN DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_favorites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    uri TEXT NOT NULL,
    artwork_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, uri)
);

CREATE INDEX idx_user_favorites_user ON user_favorites(user_id);

CREATE TABLE IF NOT EXISTS guild_playlists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(guild_id, name)
);

CREATE INDEX idx_guild_playlists_guild ON guild_playlists(guild_id);

CREATE TABLE IF NOT EXISTS playlist_tracks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    playlist_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    uri TEXT NOT NULL,
    position INTEGER NOT NULL,
    added_by INTEGER NOT NULL,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (playlist_id) REFERENCES guild_playlists(id) ON DELETE CASCADE
);

CREATE INDEX idx_playlist_tracks_playlist ON playlist_tracks(playlist_id);
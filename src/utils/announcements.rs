use crate::{database::queries, utils::constants::COLOR_INFO};
use lavalink_rs::model::track::TrackData;
use poise::serenity_prelude as serenity;

pub struct AnnouncementBuilder;

impl AnnouncementBuilder {
    pub fn now_playing(track: &TrackData, requester_id: u64) -> serenity::CreateEmbed {
        let duration = if track.info.length > 0 {
            format_duration(track.info.length)
        } else {
            "<:star:1460009999513161914> LIVE".to_string()
        };

        let mut embed = serenity::CreateEmbed::default()
            .title("<:disc:1459594790248251610> Now Playing")
            .description(format!(
                "**[{} - {}]({})**",
                track.info.author,
                track.info.title,
                track.info.uri.as_ref().unwrap_or(&String::from("#"))
            ))
            .field("Duration", duration, true)
            .field("Requested by", format!("<@{}>", requester_id), true)
            .color(COLOR_INFO)
            .timestamp(serenity::Timestamp::now());

        if let Some(artwork) = &track.info.artwork_url {
            embed = embed.thumbnail(artwork);
        }

        embed
    }

    pub fn added_to_queue(
        track: &TrackData,
        requester_id: u64,
        position: usize,
    ) -> serenity::CreateEmbed {
        let duration = if track.info.length > 0 {
            format_duration(track.info.length)
        } else {
            "<:star:1460009999513161914> LIVE".to_string()
        };

        let mut embed = serenity::CreateEmbed::default()
            .title("<:album:1459594793394114743> Added to Queue")
            .description(format!(
                "**[{} - {}]({})**",
                track.info.author,
                track.info.title,
                track.info.uri.as_ref().unwrap_or(&String::from("#"))
            ))
            .field("Duration", duration, true)
            .field("Position", format!("#{}", position), true)
            .field("Requested by", format!("<@{}>", requester_id), true)
            .color(COLOR_INFO)
            .timestamp(serenity::Timestamp::now());

        if let Some(artwork) = &track.info.artwork_url {
            embed = embed.thumbnail(artwork);
        }

        embed
    }

    pub fn track_ended(track: &TrackData) -> serenity::CreateEmbed {
        serenity::CreateEmbed::default()
            .title("<:clock:1459603728092102770> Track Ended")
            .description(format!("**{} - {}**", track.info.author, track.info.title))
            .color(COLOR_INFO)
            .timestamp(serenity::Timestamp::now())
    }

    pub fn playlist_added(
        name: &str,
        track_count: usize,
        requester_id: u64,
    ) -> serenity::CreateEmbed {
        serenity::CreateEmbed::default()
            .title("<:album:1459594793394114743> Playlist Added")
            .description(format!("**{}**", name))
            .field("Tracks Added", track_count.to_string(), true)
            .field("Requested by", format!("<@{}>", requester_id), true)
            .color(COLOR_INFO)
            .timestamp(serenity::Timestamp::now())
    }
}

fn format_duration(ms: u64) -> String {
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
    } else {
        format!("{}:{:02}", minutes, seconds % 60)
    }
}

pub async fn send_announcement(
    http: &serenity::Http,
    guild_id: serenity::GuildId,
    db: &sqlx::SqlitePool,
    announcement_channel: Option<serenity::ChannelId>,
    embed: serenity::CreateEmbed,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = queries::get_guild_config(db, guild_id.get() as i64).await?;

    if !config.announce_songs {
        return Ok(());
    }

    let channel_id = if let Some(configured_channel) = config.announce_channel_id {
        serenity::ChannelId::new(configured_channel as u64)
    } else if let Some(fallback) = announcement_channel {
        fallback
    } else {
        return Ok(());
    };

    channel_id
        .send_message(http, serenity::CreateMessage::default().embed(embed))
        .await?;

    Ok(())
}

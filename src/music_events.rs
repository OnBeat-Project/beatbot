use lavalink_rs::{hook, model::events, prelude::*};
use poise::serenity_prelude as serenity;

use crate::utils::{
    announcements::AnnouncementBuilder, constants::COLOR_ERROR, player_data::PlayerData,
};

pub fn raw_event<'a>(
    _: LavalinkClient,
    session_id: String,
    event: &'a serde_json::Value,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
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

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &events::TrackStart) {
    info!("Track started: {:?}", event.track.info.title);

    if let Some(player) = client.get_player_context(event.guild_id)
        && let Ok(data) = player.data::<PlayerData>()
    {
        let config =
            match crate::database::queries::get_guild_config(&data.db, event.guild_id.0 as i64)
                .await
            {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to get guild config: {:?}", e);
                    return;
                }
            };

        if !config.announce_songs {
            return;
        }

        let announce_channel = if let Some(channel_id) = config.announce_channel_id {
            serenity::ChannelId::new(channel_id as u64)
        } else {
            data.channel_id
        };

        let requester_id = event
            .track
            .user_data
            .as_ref()
            .and_then(|d| d["requester_id"].as_u64())
            .unwrap_or(0);

        let embed = AnnouncementBuilder::now_playing(&event.track, requester_id);

        if let Err(e) = announce_channel
            .send_message(
                data.http.as_ref(),
                serenity::CreateMessage::default().embed(embed),
            )
            .await
        {
            error!("Failed to send track start announcement: {:?}", e);
        }
    }
}

#[hook]
pub async fn track_end(client: LavalinkClient, _session_id: String, event: &events::TrackEnd) {
    info!(
        "Track ended: {:?} - Reason: {:?}",
        event.track.info.title, event.reason
    );

    if event.reason != events::TrackEndReason::Replaced
        && event.reason != events::TrackEndReason::Stopped
        && let Some(player) = client.get_player_context(event.guild_id)
        && let Ok(data) = player.data::<PlayerData>()
    {
        let config =
            match crate::database::queries::get_guild_config(&data.db, event.guild_id.0 as i64)
                .await
            {
                Ok(c) => c,
                Err(_) => return,
            };

        if !config.announce_songs {
            return;
        }

        let announce_channel = if let Some(channel_id) = config.announce_channel_id {
            serenity::ChannelId::new(channel_id as u64)
        } else {
            data.channel_id
        };

        let embed = AnnouncementBuilder::track_ended(&event.track);

        let _ = announce_channel
            .send_message(
                data.http.as_ref(),
                serenity::CreateMessage::default().embed(embed),
            )
            .await;
    }
}

#[hook]
pub async fn track_exception(
    client: LavalinkClient,
    _session_id: String,
    event: &events::TrackException,
) {
    error!(
        "Track exception: {:?} - Error: {:?}",
        event.track.info.title, event.exception
    );

    if let Some(player) = client.get_player_context(event.guild_id)
        && let Ok(data) = player.data::<PlayerData>()
    {
        let embed = serenity::CreateEmbed::default()
            .title("<:forbidden2:1459603724895780970> Playback Error")
            .description(format!(
                "Failed to play **{} - {}**\n\nError: {}",
                event.track.info.author, event.track.info.title, event.exception.message
            ))
            .color(COLOR_ERROR);

        let _ = data
            .channel_id
            .send_message(
                data.http.as_ref(),
                serenity::CreateMessage::default().embed(embed),
            )
            .await;
    }
}

#[hook]
pub async fn websocket_closed(
    _client: LavalinkClient,
    _session_id: String,
    event: &events::WebSocketClosed,
) {
    warn!(
        "Websocket closed for guild {:?} - Code: {} - Reason: {} - By remote: {}",
        event.guild_id, event.code, event.reason, event.by_remote
    );
}

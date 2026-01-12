use crate::{Context, Error, database::queries, utils::{player_data::PlayerData, autodisconnect::AutoDisconnectManager}};
use poise::serenity_prelude as serenity;
use std::{ops::Deref, sync::Arc};

pub async fn _join(
    ctx: &Context<'_>,
    guild_id: serenity::GuildId,
    channel_id: Option<serenity::ChannelId>,
) -> Result<bool, Error> {
    let lava_client = ctx.data().lavalink.clone();
    let db = ctx.data().database.pool();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if lava_client.get_player_context(guild_id).is_none() {
        let connect_to = match channel_id {
            Some(x) => x,
            None => {
                let guild = ctx.guild().unwrap().deref().clone();
                let user_channel_id = guild
                    .voice_states
                    .get(&ctx.author().id)
                    .and_then(|voice_state| voice_state.channel_id);

                match user_channel_id {
                    Some(channel) => channel,
                    None => {
                        return Err("Not in a voice channel".into());
                    }
                }
            }
        };

        let handler = manager.join_gateway(guild_id, connect_to).await;

        match handler {
            Ok((connection_info, _)) => {
                let player_data = PlayerData::new(
                    ctx.channel_id(),
                    ctx.serenity_context().http.clone(),
                    Arc::new(ctx.data().database.pool().clone()),
                );

                let player_ctx = lava_client
                    .create_player_context_with_data::<PlayerData>(
                        guild_id,
                        connection_info,
                        Arc::new(player_data),
                    )
                    .await?;

                let config = queries::get_guild_config(db, guild_id.get() as i64).await?;
                player_ctx.set_volume(config.volume as u16).await?;

                // Start auto-disconnect monitoring
                let auto_disconnect = AutoDisconnectManager::new(
                    guild_id,
                    Arc::new(db.clone()),
                    ctx.serenity_context().clone(),
                );
                auto_disconnect.start_monitoring(lava_client.clone()).await;

                return Ok(true);
            }
            Err(why) => {
                return Err(why.into());
            }
        }
    }

    Ok(false)
}

pub fn check_user_in_voice(ctx: &Context<'_>, guild_id: serenity::GuildId) -> Result<bool, Error> {
    let cache = ctx.cache();
    let user_voice = cache
        .guild(guild_id)
        .and_then(|g| {
            g.voice_states
                .get(&ctx.author().id)
                .map(|vs| vs.channel_id.clone())
        })
        .flatten();

    let bot_voice = cache
        .guild(guild_id)
        .and_then(|g| {
            g.voice_states
                .get(&cache.current_user().id)
                .map(|vs| vs.channel_id.clone())
        })
        .flatten();

    Ok(user_voice == bot_voice && user_voice.is_some())
}
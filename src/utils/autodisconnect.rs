use poise::serenity_prelude as serenity;
use std::time::Duration;
use tokio::time::sleep;

use crate::database::queries;

pub struct AutoDisconnectManager {
    guild_id: serenity::GuildId,
    db: sqlx::SqlitePool,
    ctx: serenity::Context,
}

impl AutoDisconnectManager {
    pub fn new(guild_id: serenity::GuildId, db: sqlx::SqlitePool, ctx: serenity::Context) -> Self {
        Self { guild_id, db, ctx }
    }

    pub async fn start_monitoring(self, lava_client: lavalink_rs::client::LavalinkClient) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await;

                let config =
                    match queries::get_guild_config(&self.db, self.guild_id.get() as i64).await {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Failed to get guild config for auto disconnect: {:?}", e);
                            break;
                        }
                    };

                if !config.auto_disconnect {
                    continue;
                }

                let Some(player) = lava_client.get_player_context(self.guild_id) else {
                    break;
                };

                let player_data = match player.get_player().await {
                    Ok(data) => data,
                    Err(_) => break,
                };

                // Check if nothing is playing and queue is empty
                if player_data.track.is_none() {
                    let queue_count = match player.get_queue().get_count().await {
                        Ok(count) => count,
                        Err(_) => break,
                    };

                    if queue_count == 0 {
                        // Check if voice channel is empty (only bot)
                        let is_alone = self.check_if_alone().await;

                        if is_alone {
                            info!(
                                "Auto-disconnect: Starting countdown for guild {}",
                                self.guild_id
                            );

                            let disconnect_time = config.auto_disconnect_time as u64;
                            sleep(Duration::from_secs(disconnect_time)).await;

                            // Verify conditions again after waiting
                            let config = match queries::get_guild_config(
                                &self.db,
                                self.guild_id.get() as i64,
                            )
                            .await
                            {
                                Ok(c) => c,
                                Err(_) => break,
                            };

                            if !config.auto_disconnect {
                                continue;
                            }

                            let Some(player) = lava_client.get_player_context(self.guild_id) else {
                                break;
                            };

                            let player_data = match player.get_player().await {
                                Ok(data) => data,
                                Err(_) => break,
                            };

                            let queue_count = match player.get_queue().get_count().await {
                                Ok(count) => count,
                                Err(_) => break,
                            };

                            // Only disconnect if still inactive
                            if player_data.track.is_none() && queue_count == 0 {
                                info!(
                                    "Auto-disconnect: Disconnecting from guild {} after {disconnect_time} seconds of inactivity",
                                    self.guild_id
                                );

                                let manager = match songbird::get(&self.ctx).await {
                                    Some(m) => m,
                                    None => break,
                                };

                                let _ = lava_client.delete_player(self.guild_id).await;

                                if manager.get(self.guild_id).is_some() {
                                    let _ = manager.remove(self.guild_id).await;
                                }

                                break;
                            }
                        }
                    }
                }
            }
        });
    }

    async fn check_if_alone(&self) -> bool {
        let cache = self.ctx.cache.clone(); /*match self.ctx.cache() {
        Some(c) => c,
        None => return false,
        };*/

        let guild = match cache.guild(self.guild_id) {
            Some(g) => g,
            None => return false,
        };

        let bot_id = cache.current_user().id;

        let bot_voice_channel = guild.voice_states.get(&bot_id).and_then(|vs| vs.channel_id);

        if let Some(channel_id) = bot_voice_channel {
            let members_in_channel: Vec<_> = guild
                .voice_states
                .iter()
                .filter(|(_, vs)| vs.channel_id == Some(channel_id))
                .filter(|(user_id, _)| **user_id != bot_id)
                .collect();

            // Bot is alone if there are no other members or all members are bots
            members_in_channel.is_empty()
                || members_in_channel
                    .iter()
                    .all(|(user_id, _)| cache.user(**user_id).map(|u| u.bot).unwrap_or(false))
        } else {
            false
        }
    }
}

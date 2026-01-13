use poise::serenity_prelude as serenity;
use std::time::Duration;

use crate::{
    Context, Error,
    utils::constants::{COLOR_ERROR, COLOR_SUCCESS},
};

/// Jump to a specific time in the song, in seconds.
#[poise::command(slash_command, prefix_command)]
pub async fn seek(
    ctx: Context<'_>,
    #[description = "Time to jump to (in seconds)"] time: u64,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();
    let error_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "cross").await;
    let clock_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "clock");
    let Some(player) = lava_client.get_player_context(guild_id) else {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Not Connected", error_emoji.unwrap_or_default()))
            .description("Join the bot to a voice channel first.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if now_playing.is_some() {
        player.set_position(Duration::from_secs(time)).await?;
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Jumped to {}s",
                clock_emoji.await.unwrap_or_default(),
                time
            ))
            .color(COLOR_SUCCESS);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Nothing Playing",
                error_emoji.unwrap_or_default()
            ))
            .description("No track is currently playing.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}

use crate::{
    Context, Error,
    utils::constants::{COLOR_ERROR, COLOR_SUCCESS},
};
use poise::serenity_prelude as serenity;

/// Clear the current queue.
#[poise::command(slash_command, prefix_command)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let error_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "cross").await;
    let playlist_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "album").await;
    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(guild_id) else {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Not Connected", error_emoji.unwrap_or_default()))
            .description("Join the bot to a voice channel first.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };

    player.get_queue().clear()?;

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Queue cleared!",
            playlist_emoji.unwrap_or_default()
        ))
        .color(COLOR_SUCCESS);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

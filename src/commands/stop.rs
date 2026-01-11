use crate::{
    Context, Error,
    utils::{
        constants::{COLOR_ERROR, COLOR_INFO, COLOR_SUCCESS},
        emojis::get_emoji,
    },
};
use poise::serenity_prelude as serenity;

/// Stops the playback of the current song.
#[poise::command(slash_command, prefix_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let error_emoji = get_emoji(ctx.serenity_context(), "error".to_string()).await;
    let stop_emoji = get_emoji(ctx.serenity_context(), "disc".to_string()).await;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(guild_id) else {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Not Connected", error_emoji.unwrap_or_default()))
            .description("Join the bot to a voice channel first.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if let Some(np) = now_playing {
        player.stop_now().await?;
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Stopped", stop_emoji.unwrap_or_default()))
            .description(format!("**{} - {}**", np.info.author, np.info.title))
            .color(COLOR_SUCCESS)
            .footer(serenity::CreateEmbedFooter::new(format!(
                "Stopped by {}",
                ctx.author().name
            )));

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Nothing Playing",
                error_emoji.unwrap_or_default()
            ))
            .description("No track is currently playing.")
            .color(COLOR_INFO);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}

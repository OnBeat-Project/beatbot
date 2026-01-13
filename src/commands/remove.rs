use crate::{
    Context, Error,
    utils::{
        constants::{COLOR_ERROR, COLOR_SUCCESS},
        permissions,
    },
};
use poise::serenity_prelude as serenity;

/// Remove a specific song from the queue.
#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Queue item index to remove"] index: usize,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();

    let is_dj_or_admin = permissions::check_dj_or_admin(ctx).await?;
    let check_in_voice = permissions::check_in_voice(ctx).await?;
    let error_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "cross").await;
    let success_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "check").await;
    if !is_dj_or_admin {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Permission Denied",
                error_emoji.unwrap_or_default()
            ))
            .description("You need the DJ role or admin permissions to use this command.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    if !check_in_voice {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Not in Voice Channel",
                error_emoji.unwrap_or_default()
            ))
            .description("You must be in the same voice channel as the bot.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let Some(player) = lava_client.get_player_context(guild_id) else {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Not Connected", error_emoji.unwrap_or_default()))
            .description("Join the bot to a voice channel first.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };
    let track = player.get_queue().get_track(index).await;
    player.get_queue().remove(index)?;

    let track_name = match track {
        Ok(Some(track)) => format!("{} - {}", track.track.info.author, track.track.info.title),
        Ok(None) => "Unknown Track".to_string(),
        Err(_) => "Error getting track".to_string(),
    };

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Track removed successfully",
            success_emoji.unwrap_or_default()
        ))
        .description(format!("Removed {track_name} from queue"))
        .color(COLOR_SUCCESS);

    let _ = ctx.send(poise::CreateReply::default().embed(embed)).await;

    Ok(())
}

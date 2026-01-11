use crate::{
    Context, Error,
    utils::{
        constants::{COLOR_ERROR, COLOR_SUCCESS},
        permissions,
    },
};
use poise::serenity_prelude as serenity;

/// Skip the currently playing track
#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let author_name = ctx.author().name.clone();
    let is_dj_or_admin = permissions::check_dj_or_admin(ctx).await?;
    let check_in_voice = permissions::check_in_voice(ctx).await?;
    let error_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "cross".to_string()).await;
    let skip_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "skiparrow".to_string()).await;
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

    let np = player.get_player().await?.track;

    if let Some(np) = np {
        player.skip()?;

        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Track Skipped", skip_emoji.unwrap_or_default()))
            .description(format!("**{} - {}**", np.info.author, np.info.title))
            .color(COLOR_SUCCESS)
            .footer(serenity::CreateEmbedFooter::new(format!(
                "Skipped by {}",
                author_name
            )));

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
        return Ok(());
    }

    Ok(())
}

use crate::{
    Context, Error,
    utils::{
        constants::{COLOR_ERROR, COLOR_INFO, COLOR_SUCCESS},
        voicechannel::_join,
    },
};
use poise::serenity_prelude as serenity;

/// Join the voice channel
#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let cache = ctx.cache();
    let user_voice = cache.guild(guild_id).and_then(|g| {
        g.voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id)
    });
    let me_voice = cache.guild(guild_id).and_then(|g| {
        g.voice_states
            .get(&cache.current_user().id)
            .and_then(|vs| vs.channel_id)
    });
    let error_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "cross").await;
    let success_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "check").await;
    let alert_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "caution").await;
    if user_voice.is_none() {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Cannot Join", error_emoji.unwrap_or_default()))
            .description("You must be in a voice channel to use this command.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    if me_voice == user_voice {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Already Connected",
                success_emoji.unwrap_or_default()
            ))
            .description("I am already in your voice channel!")
            .color(COLOR_INFO);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    if me_voice != user_voice && me_voice.is_some() {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Busy", alert_emoji.unwrap_or_default()))
            .description("I am currently in another voice channel.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    _join(&ctx, guild_id, user_voice).await?;

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Joined Voice Channel",
            success_emoji.unwrap_or_default()
        ))
        .description(format!("Successfully joined <#{}>", user_voice.unwrap()))
        .color(COLOR_SUCCESS)
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Requested by {}",
            ctx.author().name
        )));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

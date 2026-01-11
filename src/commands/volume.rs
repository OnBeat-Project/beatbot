use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Set track volume
#[poise::command(slash_command)]
pub async fn volume(
    ctx: Context<'_>,
    #[description = "Set track volume"] vol: u16,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let error_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "cross".to_string()).await;
    let vol_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "vol3".to_string()).await;
    let Some(player) = lava_client.get_player_context(guild_id) else {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Not Connected", error_emoji.unwrap_or_default()))
            .description("Join the bot to a voice channel first.")
            .color(0xE74C3C);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };

    player.set_volume(vol).await?;

    let embed = serenity::CreateEmbed::default()
        .title(format!("{} Volume set", vol_emoji.unwrap_or_default()))
        .description(format!("Volume set to {}%", vol))
        .color(0x1ABC9C);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

use crate::{Context, Error, utils::emojis};
use poise::serenity_prelude as serenity;

/// Leave the voice channel
#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let lava_client = ctx.data().lavalink.clone();
    let success_emoji = emojis::get_emoji(ctx.serenity_context(), "check".to_string()).await;
    lava_client.delete_player(guild_id).await?;

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
    }
    let embed = serenity::CreateEmbed::default()
        .title(format!("{} Left voice channel", success_emoji.unwrap_or_default()))
        .description("")
        .color(0x2ECC71);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
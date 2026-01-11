use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Skip the currently playing track
#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let error_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "cross".to_string()).await;
    let skip_emoji = crate::utils::emojis::get_emoji(ctx.serenity_context(), "skiparrow".to_string()).await;
    let Some(player) = lava_client.get_player_context(guild_id) else {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Not Connected", error_emoji.unwrap_or_default()))
            .description("Join the bot to a voice channel first.")
            .color(0xE74C3C);
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };
    
    let np = player.get_player().await?.track;

    if let Some(np) = np {
        player.skip()?;
        
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Track Skipped", skip_emoji.unwrap_or_default()))
            .description(format!("**{} - {}**", np.info.author, np.info.title))
            .color(0xF39C12)
            .footer(serenity::CreateEmbedFooter::new(format!("Skipped by {}", ctx.author().name)));
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Nothing Playing", error_emoji.unwrap_or_default()))
            .description("No track is currently playing.")
            .color(0x95A5A6);
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }
    
    Ok(())
}
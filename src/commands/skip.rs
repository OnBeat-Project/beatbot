use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    
    let Some(player) = lava_client.get_player_context(guild_id) else {
        let embed = serenity::CreateEmbed::default()
            .title("<:cross2:1458871191430365405> Not Connected")
            .description("Join the bot to a voice channel first.")
            .color(0xE74C3C);
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };
    
    let np = player.get_player().await?.track;

    if let Some(np) = np {
        player.skip()?;
        
        let embed = serenity::CreateEmbed::default()
            .title("<:player:1459531577494212834> Track Skipped")
            .description(format!("**{} - {}**", np.info.author, np.info.title))
            .color(0xF39C12)
            .footer(serenity::CreateEmbedFooter::new(format!("Skipped by {}", ctx.author().name)));
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let embed = serenity::CreateEmbed::default()
            .title("<:crosspixel:1458810037924401253> Nothing Playing")
            .description("No track is currently playing.")
            .color(0x95A5A6);
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }
    
    Ok(())
}
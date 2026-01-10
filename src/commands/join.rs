use crate::{Context, Error, utils::voicechannel::_join};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let cache = ctx.cache();
    let user_voice = cache.guild(guild_id).and_then(|g| {
        g.voice_states.get(&ctx.author().id).and_then(|vs| vs.channel_id)
    });
    let me_voice = cache.guild(guild_id).and_then(|g| {
        g.voice_states.get(&cache.current_user().id).and_then(|vs| vs.channel_id)
    });
    
    if user_voice.is_none() {
        let embed = serenity::CreateEmbed::default()
            .title("<:cross2:1458871191430365405> Cannot Join")
            .description("You must be in a voice channel to use this command.")
            .color(0xE74C3C);
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }
    
    if me_voice == user_voice {
        let embed = serenity::CreateEmbed::default()
            .title("<:checkpixel:1458868000001228850> Already Connected")
            .description("I am already in your voice channel!")
            .color(0x3498DB);
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }
    
    if me_voice != user_voice && me_voice.is_some() {
        let embed = serenity::CreateEmbed::default()
            .title("<:alertwindow:1458871187554959627> Busy")
            .description("I am currently in another voice channel.")
            .color(0xF39C12);
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    _join(&ctx, guild_id, user_voice).await?;
    
    let embed = serenity::CreateEmbed::default()
        .title("<:check2:1458871189874413619> Joined Voice Channel")
        .description(format!("Successfully joined <#{}>", user_voice.unwrap()))
        .color(0x2ECC71)
        .footer(serenity::CreateEmbedFooter::new(format!("Requested by {}", ctx.author().name)));
    
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}
use crate::{Context, Error, utils::voicechannel::_join};

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
        ctx.say("You must be in a voice channel to use this command.")
            .await?;
        return Ok(());
    }
    if me_voice == user_voice {
        ctx.say("I am already in your voice channel!").await?;
        return Ok(());
    }
    if me_voice != user_voice && me_voice.is_some() {
        ctx.say("I am in another voice channel.").await?;
        return Ok(());
    }

    _join(&ctx, guild_id, user_voice).await?;
    ctx.say(format!("Joined <#{}>", user_voice.unwrap())).await?;
    
    Ok(())
}
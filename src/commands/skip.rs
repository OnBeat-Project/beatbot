use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };
    let np = player.get_player().await?.track;

    if let Some(np) = np {
        player.skip()?;
        ctx.say(format!("Skipped: {} - {}", np.info.author, np.info.title)).await?;
    } else {
        ctx.say("No track is currently playing.").await?;
        return Ok(());
    }
    Ok(())
}
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Display bot info
#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::default()
        .title("BeatBot information")
        // .description("BeatBot is a music bot powered by Lavalink and written in Rust.")
        .field("Support Server", "[Click Here](https://discord.gg/Acfvz4MesZ)", true)
        .field("Invite Link", "[Click Here](https://discord.com/oauth2/authorize?client_id=1459592284202078319)", true)
        .field("Open-source", "[GitHub Repository](https://github.com/OnBeat-Project/BeatBot)", true)
        .field("Created by", "[igorwastaken](https://github.com/igorwastaken)", true)
        .field("Languages", "Rust (client) & Java (server)", true)
        .field("Libraries", "lavalink-rs, serenity, poise and songbird", true)
        .field("Icons by", "[FlatIcon](https://www.flaticon.com/) & [emoji.gg](https://emoji.gg/)", true)
        // .field("Developers", "igorwastaken, rileyscool, koala", true)
        .color(0x3498DB);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
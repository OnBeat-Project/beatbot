use crate::{
    Context, Error,
    utils::constants::{COLOR_ERROR, COLOR_INFO},
};
use poise::serenity_prelude as serenity;

/// Show the current music queue
#[poise::command(slash_command)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let album_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "album".to_string()).await;
    let player = match lava_client.get_player_context(guild_id) {
        Some(p) => p,
        None => {
            let embed = serenity::CreateEmbed::default()
                .title("No music is playing")
                .description("There is no active music player in this server.")
                .color(COLOR_ERROR);
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };
    let queue = player.get_queue();
    let player_data = player.get_player().await?;
    let max = queue.get_count().await?.min(9);

    let now_playing = match player_data.track {
        Some(track) => format!(
            "**Now Playing:** {} - {}\n\n",
            track.info.author, track.info.title
        ),
        None => String::from("No track is currently playing.\n"),
    };

    let mut tracks_desc = String::new();
    for idx in 0..max {
        if let Ok(Some(track)) = queue.get_track(idx).await {
            tracks_desc.push_str(&format!("{}. {}\n", idx + 1, track.track.info.title));
        }
    }
    let queue_list = if tracks_desc.is_empty() {
        "Queue is empty".to_string()
    } else {
        tracks_desc
    };

    let embed = serenity::CreateEmbed::default()
        .title(format!("{} Queue", album_emoji.unwrap_or_default()))
        .description(format!("{}**On Queue**:\n{}", now_playing, queue_list))
        .color(COLOR_INFO);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

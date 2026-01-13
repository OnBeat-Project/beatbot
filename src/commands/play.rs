use crate::{
    Context, Error,
    database::queries,
    utils::{
        constants::{COLOR_ERROR, COLOR_INFO, COLOR_SPOTIFY},
        voicechannel::_join,
    },
};
use ::serenity::all::Mentionable;
use lavalink_rs::prelude::*;
use poise::serenity_prelude as serenity;
use serenity::all::AutocompleteChoice;

async fn play_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<AutocompleteChoice> {
    let lava_client = ctx.data().lavalink.clone();
    let mut choices = Vec::new();

    if partial.is_empty() || partial.len() < 2 {
        return choices;
    }

    let query = if partial.starts_with("http") {
        partial.to_string()
    } else {
        match SearchEngines::Spotify.to_query(partial) {
            Ok(q) => q,
            Err(_) => return choices,
        }
    };

    if let Ok(loaded_tracks) = lava_client
        .load_tracks(ctx.guild_id().unwrap(), &query)
        .await
    {
        match loaded_tracks.data {
            Some(TrackLoadData::Search(tracks)) => {
                for track in tracks.iter().take(25) {
                    let title = format!("{} - {}", track.info.author, track.info.title);
                    let value = track.info.uri.clone().unwrap_or_default();
                    choices.push(AutocompleteChoice::new(title, value));
                }
            }
            Some(TrackLoadData::Track(track)) => {
                let title = format!("{} - {}", track.info.author, track.info.title);
                let value = track.info.uri.clone().unwrap_or_default();
                choices.push(AutocompleteChoice::new(title, value));
            }
            Some(TrackLoadData::Playlist(playlist)) => {
                let title = playlist.info.name.clone();
                choices.push(AutocompleteChoice::new(title, query));
            }
            _ => {}
        }
    }
    choices
}

fn format_duration(ms: u64) -> String {
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
    } else {
        format!("{}:{:02}", minutes, seconds % 60)
    }
}

/// Play a track or playlist
#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "The search query or URL to play"]
    #[autocomplete = "play_autocomplete"]
    term: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let has_joined = _join(&ctx, guild_id, None).await?;
    let lava_client = ctx.data().lavalink.clone();
    let error_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "cross").await;
    let _success_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "check").await;
    let playlist_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "album").await;
    let player_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "player").await;
    let spotify_emoji =
        crate::utils::emojis::get_emoji(ctx.serenity_context(), "spotify").await;
    let pool = ctx.data().database.pool();
    let guild_config = queries::get_guild_config(pool, guild_id.get() as i64).await?;
    let max_queue = guild_config.max_queue_length;

    let Some(player) = lava_client.get_player_context(guild_id) else {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Not Connected", error_emoji.unwrap_or_default()))
            .description("Join the bot to a voice channel first.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };

    let query = if term.starts_with("http") {
        term
    } else {
        SearchEngines::Spotify.to_query(&term)?
    };

    let loaded_tracks = lava_client.load_tracks(guild_id, &query).await?;

    let mut playlist_info = None;

    let mut tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }
        _ => {
            let embed = serenity::CreateEmbed::default()
                .title(format!("{} No Results", error_emoji.unwrap_or_default()))
                .description("No tracks found matching your search.")
                .color(COLOR_ERROR);

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };

    if player.get_queue().get_count().await.unwrap() as i32 >= max_queue {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Cannot add more tracks",
                error_emoji.unwrap_or_default()
            ))
            .description(format!(
                "You can't add more songs due to the queue limit of this server, which is {:?}",
                max_queue
            ))
            .color(COLOR_ERROR);
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }
    if let Some(info) = playlist_info {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Playlist Added",
                playlist_emoji.unwrap_or_default()
            ))
            .description(format!("**{}**", info.name))
            .field("Tracks Added", format!("{}", tracks.len()), true)
            .color(COLOR_INFO)
            .footer(serenity::CreateEmbedFooter::new(format!(
                "Requested by {}",
                ctx.author().name
            )))
            .thumbnail(ctx.author().avatar_url().unwrap_or_default());

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let track = &tracks[0].track;

        let duration = if track.info.length > 0 {
            format_duration(track.info.length)
        } else {
            format!("{} LIVE", player_emoji.unwrap_or_default())
        };

        let mut embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Added to Queue",
                spotify_emoji.unwrap_or_default()
            ))
            .description(format!(
                "**[{} - {}]({})**",
                track.info.author,
                track.info.title,
                track.info.uri.as_ref().unwrap_or(&String::from("#"))
            ))
            .field("Duration", duration, true)
            .field("Requested by", ctx.author().mention().to_string(), true)
            .color(COLOR_SPOTIFY);

        if let Some(artwork) = &track.info.artwork_url {
            embed = embed.thumbnail(artwork);
        }

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    for i in &mut tracks {
        i.track.user_data = Some(serde_json::json!({"requester_id": ctx.author().id.get()}));
    }

    let queue = player.get_queue();
    queue.append(tracks.into())?;

    if player.get_player().await?.track.is_none() {
        player.skip()?;
    }

    if has_joined {
        return Ok(());
    }

    if let Ok(player_data) = player.get_player().await {
        if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
            player.skip()?;
        }
    }

    Ok(())
}

use crate::{Context, Error, utils::voicechannel::_join};
use lavalink_rs::prelude::*;
use serenity::all::AutocompleteChoice;

async fn play_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<AutocompleteChoice> {
    let lava_client = ctx.data().lavalink.clone();
    let mut choices = Vec::new();
    
    // Se o partial estiver vazio, retorna vazio
    if partial.is_empty() {
        return choices;
    }
    
    // Formata a query de busca (similar ao comando play)
    let query = if partial.starts_with("http") {
        partial.to_string()
    } else {
        match SearchEngines::Spotify.to_query(partial) {
            Ok(q) => q,
            Err(_) => return choices,
        }
    };
    
    if let Ok(loaded_tracks) = lava_client.load_tracks(ctx.guild_id().unwrap(), &query).await {
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
                choices.push(AutocompleteChoice::new(title, playlist.info.name));
            }
            _ => {}
        }
    }
    choices
}

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

    let Some(player) = lava_client.get_player_context(guild_id) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let query = if term.starts_with("http") {
        term
    } else {
        //SearchEngines::YouTube.to_query(&term)?
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
            ctx.say(format!("{:?}", loaded_tracks)).await?;
            return Ok(());
        }
    };

    if let Some(info) = playlist_info {
        ctx.say(format!("Added playlist to queue: {}", info.name,))
            .await?;
    } else {
        let track = &tracks[0].track;

        if let Some(uri) = &track.info.uri {
            ctx.say(format!(
                "Added to queue: [{} - {}](<{}>)",
                track.info.author, track.info.title, uri
            ))
            .await?;
        } else {
            ctx.say(format!(
                "Added to queue: {} - {}",
                track.info.author, track.info.title
            ))
            .await?;
        }
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

use crate::{
    Context, Error,
    utils::{
        constants::{COLOR_ERROR, COLOR_INFO},
        emojis::get_emoji,
        filters::FilterPreset,
        permissions,
    },
};
use lavalink_rs::model::player::{Filters, Timescale};
use poise::serenity_prelude as serenity;

async fn filter_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<serenity::AutocompleteChoice> {
    FilterPreset::all_presets()
        .iter()
        .filter(|preset| {
            preset
                .name()
                .to_lowercase()
                .contains(&partial.to_lowercase())
        })
        .map(|preset| {
            serenity::AutocompleteChoice::new(
                preset.name().to_string(),
                preset.name().to_lowercase().replace(" ", "_"),
            )
        })
        .collect()
}

/// Apply audio filters to the current track
#[poise::command(slash_command, subcommands("apply", "list", "clear", "custom"))]
pub async fn filter(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Apply a preset filter
#[poise::command(slash_command)]
async fn apply(
    ctx: Context<'_>,
    #[description = "Filter preset to apply"]
    #[autocomplete = "filter_autocomplete"]
    preset: String,
) -> Result<(), Error> {
    let error_emoji = get_emoji(ctx.serenity_context(), "error").await;

    if !permissions::check_dj_or_admin(ctx).await? {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Permission Denied",
                error_emoji.unwrap_or_default()
            ))
            .description("You need the DJ role or admin permissions to use filters.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    if !permissions::check_in_voice(ctx).await? {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Not in Voice Channel",
                error_emoji.unwrap_or_default()
            ))
            .description("You must be in the same voice channel as the bot.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let db = ctx.data().database.pool();

    let config = crate::database::queries::get_guild_config(db, guild_id.get() as i64).await?;
    if !config.allow_filters {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} Filters Disabled", error_emoji.unwrap_or_default()))
            .description("Audio filters are disabled on this server.\nAsk an admin to enable them with `/config filters enabled:true`")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let lava_client = &ctx.data().lavalink;
    let player = lava_client
        .get_player_context(guild_id)
        .ok_or("Not connected to voice channel")?;

    let filter_preset = match preset.as_str() {
        "bass_boost" => FilterPreset::Bassboost,
        "nightcore" => FilterPreset::Nightcore,
        "vaporwave" => FilterPreset::Vaporwave,
        "8d_audio" => FilterPreset::EightD,
        "karaoke" => FilterPreset::Karaoke,
        "treble_boost" => FilterPreset::Treble,
        "vibrato" => FilterPreset::Vibrato,
        "tremolo" => FilterPreset::Tremolo,
        "pop" => FilterPreset::Pop,
        "soft" => FilterPreset::Soft,
        "electronic" => FilterPreset::Electronic,
        "rock" => FilterPreset::Rock,
        "clear_(no_filters)" => FilterPreset::Clear,
        _ => return Err("Invalid filter preset".into()),
    };

    let filters = filter_preset.to_filters();
    player.set_filters(filters).await?;

    let embed = serenity::CreateEmbed::default()
        .title(format!("{} Filter Applied", filter_preset.emoji()))
        .description(format!(
            "**{}**\n{}",
            filter_preset.name(),
            filter_preset.description()
        ))
        .color(COLOR_INFO)
        .footer(serenity::CreateEmbedFooter::new(
            "Filters may take a moment to apply",
        ));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// List all available filters
#[poise::command(slash_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let mut description = String::new();
    let filter_emoji = get_emoji(ctx.serenity_context(), "filter").await;

    for preset in FilterPreset::all_presets() {
        description.push_str(&format!(
            "{} **{}**\n{}\n\n",
            preset.emoji(),
            preset.name(),
            preset.description()
        ));
    }

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Available Audio Filters",
            filter_emoji.unwrap_or_default()
        ))
        .description(description)
        .color(COLOR_INFO)
        .footer(serenity::CreateEmbedFooter::new(
            "Use /filter apply to apply a filter",
        ));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Remove all filters
#[poise::command(slash_command)]
async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let error_emoji = get_emoji(ctx.serenity_context(), "error").await;
    let check_emoji = get_emoji(ctx.serenity_context(), "check").await;

    if !permissions::check_dj_or_admin(ctx).await? {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Permission Denied",
                error_emoji.unwrap_or_default()
            ))
            .description("You need the DJ role or admin permissions to clear filters.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let lava_client = &ctx.data().lavalink;

    let player = lava_client
        .get_player_context(guild_id)
        .ok_or("Not connected to voice channel")?;

    player.set_filters(Filters::default()).await?;

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Filters Cleared",
            check_emoji.unwrap_or_default()
        ))
        .description("All audio filters have been removed")
        .color(COLOR_INFO);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Create a custom equalizer
#[poise::command(slash_command)]
async fn custom(
    ctx: Context<'_>,
    #[description = "Speed (0.5-2.0)"] speed: Option<f64>,
    #[description = "Pitch (0.5-2.0)"] pitch: Option<f64>,
    #[description = "Bass gain (-0.25 to 1.0)"] bass: Option<f64>,
    #[description = "Treble gain (-0.25 to 1.0)"] treble: Option<f64>,
) -> Result<(), Error> {
    let error_emoji = get_emoji(ctx.serenity_context(), "error").await;
    let check_emoji = get_emoji(ctx.serenity_context(), "check").await;

    if !permissions::check_dj_or_admin(ctx).await? {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Permission Denied",
                error_emoji.unwrap_or_default()
            ))
            .description("You need the DJ role or admin permissions to use custom filters.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let db = ctx.data().database.pool();

    let config = crate::database::queries::get_guild_config(db, guild_id.get() as i64).await?;
    if !config.allow_filters {
        let embed = serenity::CreateEmbed::default()
            .title(format!(
                "{} Filters Disabled",
                error_emoji.unwrap_or_default()
            ))
            .description("Audio filters are disabled on this server.")
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let lava_client = &ctx.data().lavalink;
    let player = lava_client
        .get_player_context(guild_id)
        .ok_or("Not connected to voice channel")?;

    let mut filters = Filters::default();
    let mut changes = Vec::new();

    if speed.is_some() || pitch.is_some() {
        let speed_val = speed.unwrap_or(1.0).clamp(0.5, 2.0);
        let pitch_val = pitch.unwrap_or(1.0).clamp(0.5, 2.0);

        filters.timescale = Some(Timescale {
            speed: Some(speed_val),
            pitch: Some(pitch_val),
            rate: Some(1.0),
        });

        if let Some(s) = speed {
            changes.push(format!("Speed: {:.2}x", s));
        }
        if let Some(p) = pitch {
            changes.push(format!("Pitch: {:.2}x", p));
        }
    }

    if bass.is_some() || treble.is_some() {
        let mut bands = vec![];

        if let Some(bass_gain) = bass {
            let clamped = bass_gain.clamp(-0.25, 1.0);
            for i in 0..4 {
                bands.push((i, clamped));
            }
            changes.push(format!("Bass: {:+.2}", clamped));
        }

        if let Some(treble_gain) = treble {
            let clamped = treble_gain.clamp(-0.25, 1.0);
            for i in 10..15 {
                bands.push((i, clamped));
            }
            changes.push(format!("Treble: {:+.2}", clamped));
        }

        let equalizer_bands: Vec<lavalink_rs::model::player::Equalizer> = bands
            .into_iter()
            .map(|(band, gain)| lavalink_rs::model::player::Equalizer { band, gain })
            .collect();
        filters.equalizer = Some(equalizer_bands);
    }

    if changes.is_empty() {
        let embed = serenity::CreateEmbed::default()
            .title(format!("{} No Changes", error_emoji.unwrap_or_default()))
            .description(
                "Please specify at least one parameter:\n\
                • **speed**: Change playback speed (0.5-2.0)\n\
                • **pitch**: Change pitch (0.5-2.0)\n\
                • **bass**: Adjust bass (-0.25 to 1.0)\n\
                • **treble**: Adjust treble (-0.25 to 1.0)",
            )
            .color(COLOR_ERROR);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    player.set_filters(filters).await?;

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Custom Filter Applied",
            check_emoji.unwrap_or_default()
        ))
        .description(changes.join("\n"))
        .color(COLOR_INFO)
        .footer(serenity::CreateEmbedFooter::new(
            "Use /filter clear to remove",
        ));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

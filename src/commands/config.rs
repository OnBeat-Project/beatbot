use crate::{
    Context, Error,
    database::queries,
    utils::{
        constants::{COLOR_INFO, COLOR_SUCCESS, COLOR_WARNING},
        emojis::get_emoji,
    },
};
use ::serenity::all::Mentionable;
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    subcommands(
        "view",
        "djrole",
        "volume",
        "autodisconnect",
        "announce",
        "maxqueue",
        "filters",
        "reset"
    )
)]
pub async fn config(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// View guild configuration
#[poise::command(slash_command)]
pub async fn view(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let db = ctx.data().database.pool();
    let config = queries::get_guild_config(db, guild_id).await?;

    let dj_role = if let Some(role_id) = config.dj_role_id {
        format!("<@&{}>", role_id)
    } else {
        "Not set (Everyone can use music commands)".to_string()
    };
    let announce_channel = if let Some(channel_id) = config.announce_channel_id {
        format!("<#{}>", channel_id)
    } else {
        "Current channel".to_string()
    };
    let auto_disconnect = if config.auto_disconnect {
        format!("Enabled ({}s)", config.auto_disconnect_time)
    } else {
        "Disabled".to_string()
    };

    let embed = serenity::CreateEmbed::default()
        .title("Guild Configuration")
        .field("DJ Role", dj_role, false)
        .field("Default Volume", format!("{}%", config.volume), true)
        .field(
            "Max Queue Length",
            config.max_queue_length.to_string(),
            true,
        )
        .field("Auto Disconnect", auto_disconnect, false)
        .field("Announce Songs", config.announce_songs.to_string(), true)
        .field("Announce Channel", announce_channel, true)
        .field("Allow Filters", config.allow_filters.to_string(), true)
        .field("Allow Explicit", config.allow_explicit.to_string(), true)
        .color(COLOR_INFO);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Set the DJ role for music commands
#[poise::command(slash_command)]
async fn djrole(
    ctx: Context<'_>,
    #[description = "The DJ role (leave empty to remove)"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?.get() as i64;
    let db = ctx.data().database.pool();
    let success_emoji = get_emoji(ctx.serenity_context(), "check").await;
    let role_id = role.as_ref().map(|r| r.id.get() as i64);
    queries::update_dj_role(db, guild_id, role_id).await?;

    let description = if let Some(r) = role {
        format!("DJ role set to {}", r.mention())
    } else {
        "DJ role removed. Everyone can use music commands.".to_string()
    };

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} DJ Role Updated",
            success_emoji.unwrap_or_default()
        ))
        .description(description)
        .color(COLOR_SUCCESS);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Set default volume for this server
#[poise::command(slash_command)]
async fn volume(
    ctx: Context<'_>,
    #[description = "Default volume (0-200)"]
    #[min = 0]
    #[max = 200]
    vol: i16,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?.get() as i64;
    let db = ctx.data().database.pool();

    queries::update_volume(db, guild_id, vol).await?;

    let volume_emoji = get_emoji(ctx.serenity_context(), "vol3").await;
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Default Volume Updated",
            volume_emoji.unwrap_or_default()
        ))
        .description(format!("New tracks will play at {}%", vol))
        .color(COLOR_SUCCESS);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Configure auto disconnect when inactive
#[poise::command(slash_command)]
async fn autodisconnect(
    ctx: Context<'_>,
    #[description = "Enable/disable auto disconnect"] enabled: bool,
    #[description = "Time in seconds before disconnecting (default: 300)"] time: Option<i32>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?.get() as i64;
    let db = ctx.data().database.pool();

    queries::update_auto_disconnect(db, guild_id, enabled, time).await?;

    let description = if enabled {
        format!(
            "Bot will disconnect after {} seconds of inactivity",
            time.unwrap_or(300)
        )
    } else {
        "Auto disconnect disabled".to_string()
    };
    let clock_emoji = get_emoji(ctx.serenity_context(), "clock").await;
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Auto Disconnect Updated",
            clock_emoji.unwrap_or_default()
        ))
        .description(description)
        .color(COLOR_SUCCESS);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Configure song announcements
#[poise::command(slash_command)]
async fn announce(
    ctx: Context<'_>,
    #[description = "Enable/disable song announcements"] enabled: bool,
    #[description = "Channel for announcements (leave empty for current channel)"] channel: Option<
        serenity::GuildChannel,
    >,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?.get() as i64;
    let db = ctx.data().database.pool();

    let channel_id = channel.as_ref().map(|c| c.id.get() as i64);
    queries::update_announce_settings(db, guild_id, enabled, channel_id).await?;

    let description = if enabled {
        if let Some(ch) = channel {
            format!("Songs will be announced in {}", ch.mention())
        } else {
            "Songs will be announced in the current channel".to_string()
        }
    } else {
        "Song announcements disabled".to_string()
    };

    let song_emoji = get_emoji(ctx.serenity_context(), "song").await;
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Announcements Updated",
            song_emoji.unwrap_or_default()
        ))
        .description(description)
        .color(COLOR_SUCCESS);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Set maximum queue length
#[poise::command(slash_command)]
async fn maxqueue(
    ctx: Context<'_>,
    #[description = "Maximum number of tracks in queue (1-500)"]
    #[min = 1]
    #[max = 500]
    length: i32,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?.get() as i64;
    let db = ctx.data().database.pool();

    queries::update_max_queue_length(db, guild_id, length).await?;

    let album_emoji = get_emoji(ctx.serenity_context(), "album").await;
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Max Queue Length Updated",
            album_emoji.unwrap_or_default()
        ))
        .description(format!("Queue can now hold up to {} tracks", length))
        .color(COLOR_SUCCESS);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Allow or disallow audio filters
#[poise::command(slash_command)]
async fn filters(
    ctx: Context<'_>,
    #[description = "Allow users to use audio filters"] enabled: bool,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?.get() as i64;
    let db = ctx.data().database.pool();

    queries::update_filters_setting(db, guild_id, enabled).await?;

    let description = if enabled {
        "Users can now use audio filters (bassboost, nightcore, etc.)"
    } else {
        "Audio filters are now disabled"
    };

    let filter_emoji = get_emoji(ctx.serenity_context(), "filter").await;
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Filters Updated",
            filter_emoji.unwrap_or_default()
        ))
        .description(description)
        .color(COLOR_SUCCESS);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Reset all settings to default
#[poise::command(slash_command)]
async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?.get() as i64;
    let db = ctx.data().database.pool();

    queries::reset_guild_config(db, guild_id).await?;
    let recycle_emoji = get_emoji(ctx.serenity_context(), "recycle").await;
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} Configuration Reset",
            recycle_emoji.unwrap_or_default()
        ))
        .description("All settings have been reset to default values")
        .color(COLOR_WARNING);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

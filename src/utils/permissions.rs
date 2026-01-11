use crate::{Context, Error};
use poise::serenity_prelude as serenity;

pub async fn check_admin(ctx: Context<'_>) -> Result<bool, Error> {
    let guild = ctx.guild().ok_or("Must be in a guild")?.clone();
    let channel = ctx.guild_channel().await.unwrap();
    let member = ctx.author_member().await.ok_or("Member not found")?;

    let permissions = guild.user_permissions_in(&channel, &member);

    Ok(permissions.administrator() || permissions.manage_guild())
}

pub async fn check_dj_or_admin(ctx: Context<'_>) -> Result<bool, Error> {
    if check_admin(ctx).await? {
        return Ok(true);
    }

    let guild_id = ctx.guild_id().ok_or("Must be in guild")?;
    let db = ctx.data().database.pool();

    let guild_id_i64 = guild_id.get() as i64;
    let config = crate::database::queries::get_guild_config(db, guild_id_i64).await?;

    let Some(dj_role_id) = config.dj_role_id else {
        return Ok(true);
    };

    let member = ctx.author_member().await.ok_or("Member not found")?;

    Ok(member
        .roles
        .contains(&serenity::RoleId::new(dj_role_id as u64)))
}

pub async fn check_in_voice(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in guild")?;
    let author_id = ctx.author().id;
    let cache = ctx.cache();

    let user_voice = cache
        .guild(guild_id)
        .and_then(|g| g.voice_states.get(&author_id).and_then(|vs| vs.channel_id));

    let bot_voice = cache.guild(guild_id).and_then(|g| {
        g.voice_states
            .get(&cache.current_user().id)
            .and_then(|vs| vs.channel_id)
    });

    if bot_voice.is_none() {
        return Ok(user_voice.is_some());
    }

    Ok(user_voice == bot_voice)
}

#[macro_use]
extern crate tracing;

pub mod commands;
pub mod database;
pub mod music_events;
pub mod utils;

use ::serenity::all::ActivityData;
use database::Database;
use std::env;
use url::Url;

use lavalink_rs::{
    client::LavalinkClient, model::events, node::NodeBuilder, prelude::NodeDistributionStrategy,
};
use poise::serenity_prelude as serenity;
use songbird::SerenityInit;

use crate::utils::constants::COLOR_INFO;

pub struct Data {
    pub lavalink: LavalinkClient,
    pub database: Database,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, _ready: serenity::Ready) {
        let _ = set_guild_activity(&ctx);
    }
    async fn guild_create(
        &self,
        ctx: serenity::Context,
        guild: serenity::Guild,
        _is_new: Option<bool>,
    ) {
        let embed = serenity::CreateEmbed::default()
            .title("Guild Added")
            .description(format!(
                "Bot was added in {:?}, now bot is in {:?} servers",
                guild.name,
                ctx.cache.guild_count()
            ))
            .color(COLOR_INFO);

        let channel_id = match std::env::var("events_log_channel_id")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
        {
            Some(id) => serenity::ChannelId::new(id),
            None => {
                warn!("events_log_channel_id missing or invalid");
                return;
            }
        };
        let _activity = set_guild_activity(&ctx);
        let _ = channel_id
            .send_message(ctx.http, serenity::CreateMessage::default().embed(embed))
            .await;
    }
}

fn set_guild_activity(ctx: &serenity::Context) {
    ctx.set_activity(Some(ActivityData {
        name: format!("{} servers", ctx.cache.guild_count()),
        kind: serenity::ActivityType::Listening,
        state: format!("{} servers", ctx.cache.guild_count()).into(),
        url: Some(
            Url::parse("https://twitch.tv/notigorwastaken")
                .expect("Expected twitch.tv url or something"),
        ),
    }));
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let database = Database::new("sqlite:data/onbeat.db")
        .await
        .expect("Failed to initialize database");

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::join::join(),
                commands::play::play(),
                commands::skip::skip(),
                commands::leave::leave(),
                commands::queue::queue(),
                commands::volume::volume(),
                commands::info::info(),
                commands::config::config(),
                commands::filters::filter(),
                commands::seek::seek(),
                commands::clear::clear(),
                commands::stop::stop(),
                commands::pause::pause(),
                commands::resume::resume(),
                commands::remove::remove(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let events = events::Events {
                    raw: Some(music_events::raw_event),
                    track_start: Some(music_events::track_start),
                    track_end: Some(music_events::track_end),
                    track_exception: Some(music_events::track_exception),
                    // track_stuck: Some(music_events::track_stuck)
                    websocket_closed: Some(music_events::websocket_closed),
                    ..Default::default()
                };

                let hostname = env::var("LAVA_HOST").expect("LAVA_HOST not set");
                let password = env::var("LAVA_PASSWORD").expect("LAVA_PASSWORD not set");
                let session_id = env::var("SESSION_ID").expect("SESSION_ID not set");

                let node_local = NodeBuilder {
                    hostname,
                    is_ssl: false,
                    events: events::Events::default(),
                    password,
                    user_id: ctx.cache.current_user().id.into(),
                    session_id: Some(session_id),
                };

                let client = LavalinkClient::new(
                    events,
                    vec![node_local],
                    NodeDistributionStrategy::round_robin(),
                )
                .await;

                Ok(Data {
                    lavalink: client,
                    database,
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(
        std::env::var("BOT_TOKEN").expect("missing DISCORD_TOKEN"),
        serenity::GatewayIntents::non_privileged(),
    )
    .event_handler(Handler)
    .register_songbird()
    .framework(framework)
    .await?;

    client.start().await?;

    Ok(())
}

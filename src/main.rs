#[macro_use]
extern crate tracing;

pub mod music_events;
pub mod commands;
pub mod utils;

use std::env;

use lavalink_rs::{client::LavalinkClient, model::events, node::NodeBuilder, prelude::NodeDistributionStrategy};
use poise::{serenity_prelude as serenity};
use songbird::SerenityInit;

pub struct Data {
    pub lavalink: LavalinkClient,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    // std::env::set_var("RUST_LOG", "info,lavalink_rs=trace");
    tracing_subscriber::fmt::init();

    let framework = poise::Framework::<Data, Error>::builder()
    .options(poise::FrameworkOptions {
        commands: vec![
            commands::join::join(),
            commands::play::play(),
            commands::skip::skip(),
            commands::leave::leave(),
            commands::queue::queue(),
            commands::volume::volume(),
            commands::info::info()
        ],
        ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;

            let events = events::Events {
                raw: Some(music_events::raw_event),
                ..Default::default()
            };

            let hostname = env::var("LAVA_HOST").expect("LAVA_HOST not set");
            let password = env::var("LAVA_PASSWORD").expect("LAVA_PASSWORD not set");

            let node_local = NodeBuilder {
                hostname,
                is_ssl: false,
                events: events.clone(),
                password,
                user_id: ctx.cache.current_user().id.into(),
                session_id: None,
            };

                let client = LavalinkClient::new(
                    events,
                    vec![node_local],
                    NodeDistributionStrategy::round_robin(),
                )
                .await;

                Ok(Data { lavalink: client })
        })
    })
    .build();

    let mut client = serenity::ClientBuilder::new(
        std::env::var("BOT_TOKEN").expect("missing DISCORD_TOKEN"),
        serenity::GatewayIntents::non_privileged(),
    )
    .register_songbird()
    .framework(framework)
    .await?;

    client.start().await?;

    Ok(())
}
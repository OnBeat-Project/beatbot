use poise::serenity_prelude as serenity;

pub async fn get_emoji(context: &serenity::Context, name: &str) -> Option<String> {
    context.http.get_application_emojis().await.ok().and_then(|emojis| emojis.find(|emoji| emoji.name == name).map(String::from))
}

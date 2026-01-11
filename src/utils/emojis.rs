use poise::serenity_prelude as serenity;

pub async fn get_emoji(context: &serenity::Context, name: String) -> Option<String> {
    let emojis = context.http.get_application_emojis().await.unwrap();
    for emoji in emojis {
        if emoji.name == name {
            return Some(emoji.to_string());
        }
    }
    None
}
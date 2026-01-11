use poise::serenity_prelude as serenity;

pub struct EmbedBuilder;

impl EmbedBuilder {
    pub fn error(title: String, description: String) -> serenity::CreateEmbed {
        serenity::CreateEmbed::default()
            .title(title)
            .description(description)
            .color(crate::utils::constants::COLOR_ERROR)
    }

    pub fn success(title: String, description: String) -> serenity::CreateEmbed {
        serenity::CreateEmbed::default()
            .title(title)
            .description(description)
            .color(crate::utils::constants::COLOR_SUCCESS)
    }
}

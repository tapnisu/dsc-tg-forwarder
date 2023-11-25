use serenity::{
    model::prelude::{Embed, Guild, Message},
    prelude::Context,
};

pub fn escape_markdownv2(text: &str) -> String {
    text.chars()
        .map(|x| match x {
            '_' => "\\_".to_string(),
            '*' => "\\*".to_string(),
            '[' => "\\[".to_string(),
            ']' => "\\]".to_string(),
            '(' => "\\(".to_string(),
            ')' => "\\)".to_string(),
            '~' => "\\~".to_string(),
            '`' => "\\`".to_string(),
            '>' => "\\>".to_string(),
            '#' => "\\#".to_string(),
            '+' => "\\+".to_string(),
            '-' => "\\-".to_string(),
            '=' => "\\=".to_string(),
            '|' => "\\|".to_string(),
            '{' => "\\{".to_string(),
            '}' => "\\}".to_string(),
            '.' => "\\.".to_string(),
            '!' => "\\!".to_string(),
            _ => x.to_owned().to_string(),
        })
        .collect()
}

pub async fn format_message(ctx: Context, msg: Message) -> String {
    let author_part = if msg.is_private() {
        escape_markdownv2(&msg.author.tag())
    } else {
        format!(
            "{} / {} / {}",
            escape_markdownv2(&Guild::get(&ctx, msg.guild_id.unwrap()).await.unwrap().name),
            escape_markdownv2(
                &msg.channel_id
                    .to_channel(&ctx.http)
                    .await
                    .unwrap()
                    .guild()
                    .unwrap()
                    .name
            ),
            escape_markdownv2(&msg.author.tag()),
        )
    };

    format!(
        "\\[{}\\]: {}\n{}",
        author_part,
        escape_markdownv2(&msg.content_safe(ctx.cache)),
        msg.embeds.into_iter().map(format_embed).collect::<String>()
    )
}

pub fn format_embed(embed: Embed) -> String {
    format!(
        "Embed:\n{}{}{}{}{}{}{}{}{}",
        embed.title.map_or("".to_string(), |title| {
            format!(
                "[{}]({})\n",
                escape_markdownv2(&title),
                embed.url.unwrap_or("".to_string())
            )
        }),
        embed.description.map_or("\n".to_string(), |description| {
            format!("{}\n", escape_markdownv2(&description))
        }),
        embed.fields.iter().fold(String::new(), |acc, f| {
            acc + &format!(
                "\n{}\n{}\n",
                escape_markdownv2(&f.name.clone()),
                escape_markdownv2(&f.value.clone())
            )
        }),
        embed.thumbnail.map_or("\n".to_string(), |thumbnail| {
            format!("Thumbnail: {}\n", escape_markdownv2(&thumbnail.url))
        }),
        embed.image.map_or("\n".to_string(), |image| {
            format!("Image: {}\n", escape_markdownv2(&image.url))
        }),
        embed.video.map_or("".to_string(), |video| {
            format!("Video: {}\n", escape_markdownv2(&video.url))
        }),
        embed.author.map_or("".to_string(), |author| {
            format!("Author: {}\n", escape_markdownv2(&author.name))
        }),
        embed.footer.map_or("".to_string(), |footer| {
            format!("Footer: {}\n", escape_markdownv2(&footer.text))
        }),
        embed.timestamp.map_or("".to_string(), |timestamp| {
            format!("Timestamp: {}\n", escape_markdownv2(&timestamp))
        })
    )
}

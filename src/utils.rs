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
    format!(
        "\\[{}\\]: {}\n{}",
        if msg.is_private() {
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
        },
        escape_markdownv2(&msg.content_safe(ctx.cache)),
        msg.embeds.into_iter().map(format_embed).collect::<String>()
    )
}

pub fn format_embed(embed: Embed) -> String {
    let mut res = "Embed:\n".to_owned();

    if let Some(title) = embed.title {
        res += &format!(
            "[{}]({})\n",
            escape_markdownv2(&title),
            embed.url.unwrap_or("".to_string())
        );
    }

    if let Some(description) = embed.description {
        res += &format!("{}\n", escape_markdownv2(&description));
    }

    res += &embed.fields.iter().fold(String::new(), |acc, f| {
        acc + &format!(
            "\n{}\n{}\n",
            escape_markdownv2(&f.name.clone()),
            escape_markdownv2(&f.value.clone())
        )
    });

    if let Some(thumbnail) = embed.thumbnail {
        res += &format!("Thumbnail: {}\n", escape_markdownv2(&thumbnail.url));
    }
    if let Some(image) = embed.image {
        res += &format!("Image: {}\n", escape_markdownv2(&image.url));
    }
    if let Some(video) = embed.video {
        res += &format!("Video: {}\n", escape_markdownv2(&video.url));
    }
    if let Some(author) = embed.author {
        res += &format!("Author: {}\n", escape_markdownv2(&author.name));
    }
    if let Some(footer) = embed.footer {
        res += &format!("Footer: {}\n", escape_markdownv2(&footer.text));
    }
    if let Some(timestamp) = embed.timestamp {
        res += &format!("Timestamp: {}\n", escape_markdownv2(&timestamp));
    }

    res
}

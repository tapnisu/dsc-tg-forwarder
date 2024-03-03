use serenity::{
    model::prelude::{Embed, Guild, Message},
    prelude::Context,
};

pub fn escape_markdown_v2(text: &str) -> String {
    text.chars()
        .map(|x| match x {
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '=' | '|'
            | '{' | '}' | '.' | '!' => format!("\\{x}"),
            _ => x.to_owned().to_string(),
        })
        .collect()
}

pub async fn format_message(ctx: &Context, msg: &Message, hide_username: bool) -> String {
    let message_content = format!(
        "{}\n{}",
        escape_markdown_v2(&msg.content_safe(ctx.to_owned().cache)),
        msg.embeds.iter().map(format_embed).collect::<String>()
    );

    if hide_username {
        return message_content;
    }

    let author_part = if msg.is_private() {
        escape_markdown_v2(&msg.author.tag())
    } else {
        format!(
            "{} / {} / {}",
            escape_markdown_v2(&Guild::get(&ctx, msg.guild_id.unwrap()).await.unwrap().name),
            escape_markdown_v2(
                &msg.channel_id
                    .to_channel(&ctx.http)
                    .await
                    .unwrap()
                    .guild()
                    .unwrap()
                    .name
            ),
            escape_markdown_v2(&msg.author.tag()),
        )
    };

    format!("\\[{}\\]: {}", author_part, message_content)
}

pub fn format_embed(embed: &Embed) -> String {
    let title = embed.title.to_owned().map_or("".to_string(), |title| {
        format!(
            "[{}]({})\n",
            escape_markdown_v2(&title),
            embed.url.to_owned().unwrap_or("".to_string())
        )
    });

    let description = embed
        .description
        .to_owned()
        .map_or("\n".to_string(), |description| {
            format!("{}\n", escape_markdown_v2(&description))
        });

    let fields = embed.fields.iter().fold(String::new(), |acc, f| {
        acc + &format!(
            "\n{}\n{}\n",
            escape_markdown_v2(&f.name.clone()),
            escape_markdown_v2(&f.value.clone())
        )
    });

    let thumbnail = embed
        .thumbnail
        .to_owned()
        .map_or("\n".to_string(), |thumbnail| {
            format!("Thumbnail: {}\n", escape_markdown_v2(&thumbnail.url))
        });

    let image = embed.image.to_owned().map_or("\n".to_string(), |image| {
        format!("Image: {}\n", escape_markdown_v2(&image.url))
    });

    let video = embed.video.to_owned().map_or("".to_string(), |video| {
        format!("Video: {}\n", escape_markdown_v2(&video.url))
    });

    let author = embed.author.to_owned().map_or("".to_string(), |author| {
        format!("Author: {}\n", escape_markdown_v2(&author.name))
    });

    let footer = embed.footer.to_owned().map_or("".to_string(), |footer| {
        format!("Footer: {}\n", escape_markdown_v2(&footer.text))
    });

    let timestamp = embed
        .timestamp
        .to_owned()
        .map_or("".to_string(), |timestamp| {
            format!("Timestamp: {}\n", escape_markdown_v2(&timestamp))
        });

    format!(
        "Embed:\n{}{}{}{}{}{}{}{}{}",
        title, description, fields, thumbnail, image, video, author, footer, timestamp
    )
}

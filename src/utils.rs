use serenity::{
    model::prelude::{Embed, Guild, Message},
    prelude::Context,
};

pub trait EscapeMarkdownV2 {
    /// Escapes Telegrams Markdown V2 characters
    ///
    /// ```
    /// use dsc_tg_forwarder::EscapeMarkdownV2;
    ///
    /// let result = "*Hello world!*".to_owned().escape_markdown_v2();
    ///
    /// assert_eq!(result, "\\*Hello world\\!\\*".to_owned());
    /// ```
    fn escape_markdown_v2(&self) -> String;
}

impl EscapeMarkdownV2 for String {
    fn escape_markdown_v2(&self) -> String {
        self.chars()
            .map(|x| match x {
                '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '='
                | '|' | '{' | '}' | '.' | '!' => format!("\\{x}"),
                _ => x.to_owned().to_string(),
            })
            .collect()
    }
}

pub async fn format_message(ctx: &Context, msg: &Message, hide_username: bool) -> String {
    let message_content = format!(
        "{}\n{}",
        msg.content_safe(ctx.to_owned().cache).escape_markdown_v2(),
        msg.embeds.iter().map(format_embed).collect::<String>()
    );

    if hide_username {
        return message_content;
    }

    let author_part = if msg.is_private() {
        msg.author.tag().escape_markdown_v2()
    } else {
        format!(
            "{} / {} / {}",
            Guild::get(&ctx, msg.guild_id.unwrap())
                .await
                .unwrap()
                .name
                .escape_markdown_v2(),
            &msg.channel_id
                .to_channel(&ctx.http)
                .await
                .unwrap()
                .guild()
                .unwrap()
                .name
                .escape_markdown_v2(),
            msg.author.tag().escape_markdown_v2(),
        )
    };

    format!("\\[{}\\]: {}", author_part, message_content)
}

pub fn format_embed(embed: &Embed) -> String {
    let title = embed.title.to_owned().map_or("".to_string(), |title| {
        format!(
            "[{}]({})\n",
            title.escape_markdown_v2(),
            embed.url.to_owned().unwrap_or_default()
        )
    });

    let description = embed
        .description
        .to_owned()
        .map_or("\n".to_string(), |description| {
            format!("{}\n", description.escape_markdown_v2())
        });

    let fields = embed.fields.iter().fold(String::new(), |acc, f| {
        acc + &format!(
            "\n{}\n{}\n",
            f.name.escape_markdown_v2(),
            f.value.escape_markdown_v2()
        )
    });

    let thumbnail = embed
        .thumbnail
        .to_owned()
        .map_or("\n".to_string(), |thumbnail| {
            format!("Thumbnail: {}\n", &thumbnail.url.escape_markdown_v2())
        });

    let image = embed.image.to_owned().map_or("\n".to_string(), |image| {
        format!("Image: {}\n", image.url.escape_markdown_v2())
    });

    let video = embed.video.to_owned().map_or("".to_string(), |video| {
        format!("Video: {}\n", video.url.escape_markdown_v2())
    });

    let author = embed.author.to_owned().map_or("".to_string(), |author| {
        format!("Author: {}\n", author.name.escape_markdown_v2())
    });

    let footer = embed.footer.to_owned().map_or("".to_string(), |footer| {
        format!("Footer: {}\n", footer.text.escape_markdown_v2())
    });

    let timestamp = embed
        .timestamp
        .to_owned()
        .map_or("".to_string(), |timestamp| {
            format!("Timestamp: {}\n", timestamp.escape_markdown_v2())
        });

    format!(
        "Embed:\n{}{}{}{}{}{}{}{}{}",
        title, description, fields, thumbnail, image, video, author, footer, timestamp
    )
}

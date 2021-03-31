use std::env;

use anyhow::Result;
use emojis::Emoji;

fn to_item(emoji: &Emoji) -> powerpack::Item {
    let title = format!("{} {}", emoji.as_str(), emoji.name());
    powerpack::Item::new(title)
        .subtitle(
            emoji
                .shortcode()
                .map(|s| format!(":{}:", s))
                .unwrap_or(format!("")),
        )
        .arg(emoji.as_str())
}

pub fn output(emojis: impl Iterator<Item = &'static Emoji>) -> Result<()> {
    Ok(powerpack::output(emojis.map(to_item))?)
}

fn main() -> Result<()> {
    match env::args().nth(1) {
        Some(query) => output(emojis::search(&query.trim())),
        None => output(emojis::iter()),
    }
}

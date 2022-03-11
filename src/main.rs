mod icon;

use std::env;
use std::path::PathBuf;

use anyhow::Result;
use emojis::Emoji;

fn get_title_and_icon<'a>(emoji: &'a Emoji) -> (powerpack::String<'a>, powerpack::Icon) {
    let name: String = emoji
        .chars()
        .map(|c| format!("-{:05X}", c as u32))
        .collect();
    let path = PathBuf::from(format!("images/emoji{}.png", name));
    if path.exists() {
        (emoji.name().into(), powerpack::Icon::Path(path))
    } else {
        (
            format!("{} {}", emoji.as_str(), emoji.name()).into(),
            powerpack::Icon::from_file_type("public.png"),
        )
    }
}

fn to_item(emoji: &Emoji) -> powerpack::Item {
    let (title, icon) = get_title_and_icon(emoji);
    powerpack::Item::new(title)
        .subtitle(
            emoji
                .shortcode()
                .map(|s| format!(":{}:", s))
                .unwrap_or(format!("")),
        )
        .arg(emoji.as_str())
        .icon(icon)
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

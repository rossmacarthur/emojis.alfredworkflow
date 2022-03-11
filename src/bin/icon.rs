use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use anyhow::{bail, ensure, Context, Result};
use emojis::Emoji;
use ttf_parser::{cmap, GlyphId};

fn lookup_png<'a>(emoji: &Emoji, face: &'a ttf_parser::Face<'a>) -> Result<&'a [u8]> {
    let chars: Vec<_> = emoji
        .chars()
        .filter(|c| !['\u{fe0f}', '\u{20E3}'].contains(c))
        .collect();
    let glyph_id = match chars.as_slice() {
        &[c, variation] => face.glyph_variation_index(c, variation),
        &[c] => face.glyph_index(c),
        _ => bail!("emoji has more than 2 code points"),
    }
    .context("no glyph index found")?;

    let image = face
        .glyph_raster_image(glyph_id, u16::MAX)
        .context("no image found")?;

    Ok(image.data)
}

fn write_png<'a>(c: char, face: &'a ttf_parser::Face<'a>, path: &Path) -> Result<()> {
    let glyph_id = face.glyph_index(c).context("no glyph index found")?;

    let image = face
        .glyph_raster_image(GlyphId(0x00020), u16::MAX)
        .context("no image found")?;

    fs::write(path, image.data)?;

    Ok(())
}

fn glyph_ids(face: &ttf_parser::Face<'_>) -> Result<Vec<(char, GlyphId)>> {
    let mut subtables: Vec<_> = face.character_mapping_subtables().collect();
    ensure!(subtables.len() == 1, "expected one subtable");
    let subtable = subtables.pop().unwrap();
    println!("{:?}", subtable.format());
    let mut glyph_ids = Vec::new();
    subtable.codepoints(|c| {
        glyph_ids.push(
            subtable
                .glyph_index(c)
                .map(|i| (std::char::from_u32(c).unwrap(), i)),
        )
    });
    Ok(glyph_ids.into_iter().filter_map(|x| x).collect())
}

// /// Initialize all emoji icons.
// pub fn init() -> Result<()> {
//     let data = fs::read("/System/Library/Fonts/Apple Symbols.ttf")?;
//     let symbol_face = ttf_parser::Face::from_slice(&data, 0)?;

//     // write_png('\u{23}', &emoji_face, Path::new("1.png"))?;

//     // for (cp, glyph_id) in glyph_ids(&emoji_face)?.into_iter().take(50) {
//     //     println!("0x{:05X} 0x{:05X}", cp, glyph_id.0);
//     // }

//     for emoji in emojis::iter() {
//         let icon = match lookup_png(emoji, &face) {
//             Ok(icon) => Some(icon),
//             Err(err) => match lookup_png(emoji, &symbol_face) {
//                 Ok(icon) => Some(icon),
//                 Err(err) => {
//                     eprintln!("warning: {}, {}", emoji.as_str(), err);
//                     None
//                 }
//             },
//         };

//         if let Some(icon) = icon {
//             let name: String = emoji
//                 .chars()
//                 .map(|c| format!("-{:05X}", c as u32))
//                 .collect();

//             fs::write(format!("workflow/images/emoji{}.png", name), icon)?
//         }
//     }

//     Ok(())
// }

#[derive(Debug, Clone, Hash)]
struct Glyph {
    face: &'static str,
    id: u16,
    ch: char,
    data: Vec<u8>,
}

fn main() -> Result<()> {
    let mut glyphs = HashMap::new();
    let paths = [
        "/System/Library/Fonts/Apple Color Emoji.ttc",
        "/System/Library/Fonts/Apple Symbols.ttf",
    ];

    for path in &paths {
        let data = fs::read(path)?;
        let face = ttf_parser::Face::from_slice(&data, 0)?;

        let mut subtables: Vec<_> = face.character_mapping_subtables().collect();
        ensure!(subtables.len() == 1, "expected one subtable");
        let subtable = subtables.pop().unwrap();

        subtable.codepoints(|c| {
            let id = subtable.glyph_index(c).unwrap();
            let ch = std::char::from_u32(c).unwrap();
            if let Some(data) = face.glyph_raster_image(id, u16::MAX).map(|i| i.data) {
                glyphs.insert(
                    ch,
                    Glyph {
                        face: path,
                        id: id.0,
                        ch,
                        data: data.to_vec(),
                    },
                );
            } else {
                // eprintln!("warning: {}, 0x{:05X} has no image data", ch, c);
            }
        });
    }

    let emojis: HashSet<&Emoji> = emojis::iter()
        .filter_map(|emoji| {
            let chars: Vec<_> = emoji
                .chars()
                .filter(|c| !['\u{fe0f}', '\u{20E3}'].contains(c))
                .collect();

            if chars.len() == 1 && glyphs.contains_key(&chars[0]) {
                glyphs.remove(&chars[0]);
                None
            } else {
                Some(emoji)
            }
        })
        .collect();

    println!("{} glyphs remaining", glyphs.len());
    println!("{} emojis remaining", emojis.len());

    for emoji in emojis {
        println!("{}", emoji.name());
    }

    Ok(())

    // list all chars and glyph ids in each font

    // try to load png for all emojis
    //  Ok => mark char and glyph id as *used*
    //  Err() => add to list of missing glyphs
    //
    // print out all emojis missing glyphs
    // write out all glyphs that do not have a corresponding emoji
}

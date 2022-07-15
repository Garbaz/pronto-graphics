//! Not my finest bout of Rust coding, but the combination of storing a file
//! inside the library's binary with [`RustEmbed`] and SFML's
//! [`Font::from_memory`] function being a tad picky, some hackery was required.
//! Though perhaps this could be done a bit cleaner at some point.

use rust_embed::RustEmbed;
use sfml::{graphics::Font as SfmlFont, SfBox};

//
//
// ---------------- Default Font ----------------

#[derive(RustEmbed)]
#[folder = "res/fonts/"]
struct DefaultFontFile;
static mut DEFAULT_FONT_BINARY_DATA: Option<Vec<u8>> = None;
static mut DEFAULT_FONT: Option<SfBox<SfmlFont>> = None;
static mut TRIED_TO_LOAD_DEFAULT_FONT: bool = false;

pub fn init_default_font() {
    unsafe {
        if !TRIED_TO_LOAD_DEFAULT_FONT {
            TRIED_TO_LOAD_DEFAULT_FONT = true;

            // We have to create a static copy of the font data in memory,
            // because otherwise [`Font::from_memory`] will quietly fail for unknown reasons.
            DEFAULT_FONT_BINARY_DATA =
                DefaultFontFile::get("ProcessingSansPro-Regular.ttf")
                    .map(|binary| binary.data.to_vec());

            // Try to initialize default font once. If this fails, we just don't show any text,
            // unless the user has loaded their own font.
            // But that should never happen unless there is a bug in the code, since the default
            // font is stored inside the library's binary.
            DEFAULT_FONT = DEFAULT_FONT_BINARY_DATA
                .as_ref()
                .and_then(|bin| SfmlFont::from_memory(&bin[..])).or_else(|| {
                    eprintln!("Failed to load default font. This should not have happened.");
                    None
                });
        }
    }
}

pub fn default_font() -> Option<&'static SfBox<SfmlFont>> {
    unsafe { DEFAULT_FONT.as_ref() }
}

//
//
// ---------------- Font Store ----------------

/// A global static array containing all fonts that have been loaded during the runtime of the program.
/// Should not be accesses directly outside this module.
/// # Excuses
/// See [`crate::texture::TEXTURE_STORE`].
static mut FONT_STORE: Option<Vec<SfBox<SfmlFont>>> = None;
pub fn init_font_store() {
    unsafe {
        if FONT_STORE.is_none() {
            FONT_STORE = Some(Vec::new());
        }
    }
}

pub fn font_store(font: Font) -> Option<&'static SfBox<SfmlFont>> {
    unsafe {
        if let Some(fonts) = &FONT_STORE {
            Some(&fonts[font.index])
        } else {
            None
        }
    }
}

pub fn font_store_add(font: SfBox<SfmlFont>) -> Option<Font> {
    unsafe {
        if let Some(fonts) = &mut FONT_STORE {
            fonts.push(font);
            Some(Font {
                index: fonts.len() - 1,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Font {
    pub index: usize,
}

impl Font {
    pub fn name(&self) -> String {
        font_store(*self)
            .map(|f| f.info().family)
            .unwrap_or_else(|| String::from(""))
    }
}

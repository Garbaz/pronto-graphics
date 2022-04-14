//! Not my finest bout of Rust coding, but the conbination of storing a file
//! inside the library's binary with `RustEmbed` and SFML's
//! `Font::from_memory` function being a tad picky, some hackery was required.
//! Though perhaps this could be done a bit cleaner at some point.

use rust_embed::RustEmbed;
use sfml::{graphics::Font, SfBox};

#[derive(RustEmbed)]
#[folder = "res/fonts/"]
struct DefaultFontFile;
static mut DEFAULT_FONT_BINARY_DATA: Option<Vec<u8>> = None;
static mut DEFAULT_FONT: Option<SfBox<Font>> = None;
static mut TRIED_TO_LOAD_DEFAULT_FONT: bool = false;

pub fn init_default_font() {
    unsafe {
        if !TRIED_TO_LOAD_DEFAULT_FONT {
            TRIED_TO_LOAD_DEFAULT_FONT = true;

            // We have to create a static copy of the font data in memory,
            // because otherwise `Font::from_memory` will quietly fail for unknown reasons.
            DEFAULT_FONT_BINARY_DATA =
                DefaultFontFile::get("ProcessingSansPro-Regular.ttf")
                    .and_then(|binary| Some(binary.data.to_vec()));

            // Try to initialize default font once. If this fails, we just don't show any text,
            // unless the user has loaded their own font.
            // But that should never happen unless there is a bug in the code, since the default
            // font is stored inside the library's binary.
            DEFAULT_FONT = DEFAULT_FONT_BINARY_DATA
                .as_ref()
                .and_then(|bin| Font::from_memory(&bin[..])).or_else(|| {
                    eprintln!("Failed to load default font. This should not have happened.");
                    None
                });
        }
    }
}

pub fn default_font() -> Option<&'static SfBox<Font>> {
    unsafe { DEFAULT_FONT.as_ref() }
}

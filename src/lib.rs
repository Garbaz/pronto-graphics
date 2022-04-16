//! A Rust library for simple and straightforward drawing of graphics, similar to [Processing](https://processing.org/).
//! 
//! All interaction with the library happens through an instance of [`Window`].
//! 
//! Just like [Processing](https://processing.org/), Pronto Graphics uses a hidden persistent state for drawing settings
//! like colors, line thickness and font, which can be set at any point and will affect all later draw calls,
//! either until the end of the frame, or until changed (See the individual documentation for details).
//! 
//! ## Prerequisites
//! 
//! Since Pronto Graphics uses [SFML](https://www.sfml-dev.org/), specifically the [SFML bindings for Rust](https://docs.rs/sfml/latest/sfml/index.html),
//! it's prerequisites are [the same as the SFML Rust bindings](https://docs.rs/sfml/latest/sfml/index.html#prerequisites).
//! 
//! This might change in future versions.
//! 
//! ## Usage
//! 
//! Commonly you would have something like this in your `fn main()`:
//! 
//! ```
//! let mut pg = Window::new(800, 600, "Window Title");
//! // or
//! //let mut pg = Window::new_fullscreen();
//! 
//! loop {
//!     // --- Draw calls, etc. ---
//! 
//!     pg.update();
//! }
//! ```
//! 
//! ## Thread safety
//! 
//! Pronto Graphics is not thread safe, both due to it's own internal structure and the fact it uses
//! SFML for drawing, which already [isn't thread safe](https://docs.rs/sfml/latest/sfml/index.html#-thread-safety-warning-).
//! As long as you only use Pronto Graphics in your main thread however, it should be fine to have parallel non-graphics threads.

mod color;
mod input;
mod shape;
mod window;
mod texture;
mod font;
pub use color::Color;
pub use sfml::window::{mouse::Button, Key};
pub use texture::Texture;
pub use window::Window;

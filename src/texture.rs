use sfml::graphics::Texture as SfmlTexture;
use sfml::SfBox;

pub static mut TEXTURES: Option<Vec<SfBox<SfmlTexture>>> = None;

/// A texture object returned by `Window.load_texture(...)`, that can be passed to `Window.texture(...)` to draw the texture to the screen.
/// # Examples
/// ```
/// let mut pg = Window::new_fullscreen();
/// let my_texture = pg.load_texture("my_texture.png").unwrap();
/// loop {
///     pg.texture_((100., 250.), my_texture, 100.);
///
///     pg.update();
/// }
/// ```
#[derive(Clone, Copy)]
pub struct Texture {
    pub index: usize,
}


impl Texture {
    fn sfml_texture(&self) -> Option<&SfmlTexture> {
        unsafe {
            if let Some(textures) = &TEXTURES {
                Some(&(*textures[self.index]))
            } else {
                None
            }
        }
    }

    /// The width of the texture in pixels.
    pub fn width(&self) -> u32 {
        self.sfml_texture()
            .and_then(|t| Some(t.size().x))
            .unwrap_or(0)
    }

    /// The height of the texture in pixels.
    pub fn height(&self) -> u32 {
        self.sfml_texture()
            .and_then(|t| Some(t.size().y))
            .unwrap_or(0)
    }

    /// The aspect ratio of the texture.
    /// (`.width()/.height()`)
    pub fn aspect(&self) -> f32 {
        let h = self.height();
        if h > 0 {
            (self.width() as f32) / (h as f32)
        } else {
            0.
        }
    }
}

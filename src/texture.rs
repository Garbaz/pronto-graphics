use sfml::graphics::Texture as SfmlTexture;
use sfml::SfBox;

/// A global static array containing all textures that have been loaded during the runtime of the program.
/// Should not be accesses directly in most circumstances.
/// # Excuses
/// So, this isn't a pretty look, I know.
/// The problem is, as far as I can tell, there is no way to tell Rust's lifetime rules that I just want to give
/// an existing `RectangleShape` a reference to a texture for only the scope of one `Window.texture(...)` call.
/// Neither can I create a bundle of the Shape and it's Texture in one object to align their lifetimes.
/// Therefore, to ensure than all Textures always outlive the Shape that potentionally is referecing them,
/// I have to make the textures `static`. Hence, this ungodly bunch of code.
/// If there is a prettier solution that I have overlooked, this should be changed.
static mut TEXTURE_STORE: Option<Vec<SfBox<SfmlTexture>>> = None;
pub fn init_texture_array() {
    unsafe {
        if let None = TEXTURE_STORE {
            TEXTURE_STORE = Some(Vec::new());
        }
    }
}

pub fn texture_array(index: usize) -> Option<&'static SfBox<SfmlTexture>> {
    unsafe {
        if let Some(textures) = &TEXTURE_STORE {
            Some(&textures[index])
        } else {
            None
        }
    }
}

pub fn texture_array_add(texture: SfBox<SfmlTexture>) -> Option<Texture> {
    unsafe {
        if let Some(textures) = &mut TEXTURE_STORE {
            textures.push(texture);
            Some(Texture {
                index: textures.len() - 1,
            })
        } else {
            None
        }
    }
}

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
            if let Some(textures) = &TEXTURE_STORE {
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

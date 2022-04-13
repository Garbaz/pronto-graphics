use sfml::graphics::Texture as SfmlTexture;
use sfml::SfBox;

pub static mut TEXTURES: Option<Vec<SfBox<SfmlTexture>>> = None;

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

    pub fn width(&self) -> u32 {
        self.sfml_texture()
            .and_then(|t| Some(t.size().x))
            .unwrap_or(0)
    }

    pub fn height(&self) -> u32 {
        self.sfml_texture()
            .and_then(|t| Some(t.size().y))
            .unwrap_or(0)
    }

    pub fn aspect(&self) -> f32 {
        let h = self.height();
        if h > 0 {
            (self.width() as f32) / (h as f32)
        } else {
            0.
        }
    }
}

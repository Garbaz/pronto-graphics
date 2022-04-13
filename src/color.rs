use sfml::graphics::Color as SfmlColor;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    sfml_color: SfmlColor,
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color {
            sfml_color: SfmlColor::rgb(red, green, blue),
        }
    }

    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            sfml_color: SfmlColor::rgba(red, green, blue, alpha),
        }
    }

    pub fn red(&self) -> u8 {
        self.sfml_color.red()
    }

    pub fn green(&self) -> u8 {
        self.sfml_color.green()
    }

    pub fn blue(&self) -> u8 {
        self.sfml_color.blue()
    }

    pub fn alpha(&self) -> u8 {
        self.sfml_color.alpha()
    }
}

impl From<Color> for SfmlColor {
    fn from(color: Color) -> Self {
        color.sfml_color
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self::rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8, u8)) -> Self {
        Self::rgba(rgb.0, rgb.1, rgb.2, rgb.3)
    }
}

#[derive(Clone, Copy)]
pub struct ColorState {
    pub background_color: Color,
    pub fill_color: Color,
    pub outline_color: Color,
}

impl Color {
    pub const TRANSPARENT: Color = Color::rgba(0x00, 0x00, 0x00, 0);
    pub const BLACK: Color = Color::rgb(0x00, 0x00, 0x00);
    pub const WHITE: Color = Color::rgb(0xFF, 0xFF, 0xFF);
    pub const GRAY: Color = Color::rgb(0x80, 0x80, 0x80);
    pub const DARK_GRAY: Color = Color::rgb(0x40, 0x40, 0x40);
    pub const LIGHT_GRAY: Color = Color::rgb(0xC0, 0xC0, 0xC0);
    pub const RED: Color = Color::rgb(0xFF, 0x00, 0x00);
    pub const GREEN: Color = Color::rgb(0x00, 0xFF, 0xC0);
    pub const BLUE: Color = Color::rgb(0x00, 0x00, 0xFF);
}

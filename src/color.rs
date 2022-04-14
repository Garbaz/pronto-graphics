use sfml::graphics::Color as SfmlColor;

/// A object representing a color in RGBA32 format.
/// Red/Green/Blue/Alpha each range from 0 to 255.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    sfml_color: SfmlColor,
}

impl Color {
    /// Create a `Color` from `red`, `green` and `blue`.
    /// # Examples
    /// ```
    /// let color = Color::rgb(0x1D, 0x37, 0x85);
    /// ```
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color {
            sfml_color: SfmlColor::rgb(red, green, blue),
        }
    }

    /// Create a `Color` from `red`, `green`, `blue` and `alpha`.
    /// # Examples
    /// ```
    /// let color = Color::rgba(0x1D, 0x37, 0x85, 127);
    /// ```
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            sfml_color: SfmlColor::rgba(red, green, blue, alpha),
        }
    }

    /// Create a `Color` from an existing color with alpha set to `alpha`.
    /// # Examples
    /// ```
    /// let color = Color::BLUE.with_alpha(127);
    /// ```
    pub fn with_alpha(self, alpha: u8) -> Self {
        Self::rgba(self.red(), self.green(), self.blue(), alpha)
    }

    /// The `red` component of the color.
    pub fn red(&self) -> u8 {
        self.sfml_color.red()
    }

    /// The `green` component of the color.
    pub fn green(&self) -> u8 {
        self.sfml_color.green()
    }

    /// The `blue` component of the color.
    pub fn blue(&self) -> u8 {
        self.sfml_color.blue()
    }

    /// The `alpha` component of the color.
    pub fn alpha(&self) -> u8 {
        self.sfml_color.alpha()
    }

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
    pub fill_color: Color,
    pub outline_color: Color,
    pub text_color: Color,
}

impl Default for ColorState {
    fn default() -> Self {
        Self {
            fill_color: Color::BLACK,
            outline_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
        }
    }
}

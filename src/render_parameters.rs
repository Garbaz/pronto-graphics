use crate::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RenderParameterState {
    pub fill_color: Color,
    pub outline_color: Color,
    pub line_color: Color,
    pub font_color: Color,
    pub font_size: u32,
}

impl Default for RenderParameterState {
    fn default() -> Self {
        Self {
            fill_color: Color::BLACK,
            outline_color: Color::TRANSPARENT,
            line_color: Color::BLACK,
            font_color: Color::BLACK,
            font_size: 16,
        }
    }
}

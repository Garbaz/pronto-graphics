use sfml::graphics::{CircleShape, RectangleShape, Text};

use crate::color::ColorState;

pub enum Shapes {
    Circle {
        radius: f32,
    },
    Rectangle {
        width: f32,
        height: f32,
    },
    Texture {
        index: usize,
        width: f32,
        height: f32,
    },
    Text {
        string : String,
    }
}

pub struct RenderTask {
    pub pos: (f32, f32),
    pub shape: Shapes,
    pub color_state: ColorState,
}

pub struct ShapeStore<'a> {
    pub circle: CircleShape<'a>,
    pub rectangle: RectangleShape<'a>,
    pub texture: RectangleShape<'a>,
    pub text: Option<Text<'a>>,
}

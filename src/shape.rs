use sfml::graphics::{CircleShape, RectangleShape, Text};

use crate::{render_parameters::RenderParameterState, Font, Texture};

pub enum Shapes {
    Circle {
        radius: f32,
    },
    Rectangle {
        width: f32,
        height: f32,
    },
    Texture {
        texture: Texture,
        width: f32,
        height: f32,
    },
    Text {
        string: String,
        font: Option<Font>,
    },
    Lines {
        coords: Vec<(f32, f32)>,
    },
}

pub struct RenderTask {
    pub pos: (f32, f32),
    pub shape: Shapes,
    pub render_parameter_state: RenderParameterState,
}

pub struct ShapeStore<'a> {
    pub circle: CircleShape<'a>,
    pub rectangle: RectangleShape<'a>,
    pub texture: RectangleShape<'a>,
    pub text: Option<Text<'a>>,
}

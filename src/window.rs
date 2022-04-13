use std::{collections::VecDeque, process::exit};

use sfml::{
    graphics::{
        CircleShape, RectangleShape, RenderTarget, RenderWindow, Shape,
        Texture as SfmlTexture, Transformable,
    },
    system::Clock,
    window::{mouse::Button, Event, Key, Style, VideoMode},
};

use crate::{
    color::{Color, ColorState},
    input::InputState,
    shape::{RenderTask, ShapeStore, Shapes},
    texture::{Texture, TEXTURES},
};

/// The core type of the Pronto Graphics library.
/// All drawing and keyboard/mouse interaction happens through an instance of `Window`.
/// It has to be updated every frame  with `.update()` for drawings to be rendered and the keyboard/mouse state to be updated.
/// 
/// # Examples
/// ```
/// let mut pg = Window::new(800, 600, "Window Title"); // Create window
/// loop {
///     pg.circle((200,200), 50); // Draw to it
///     pg.update(); // Update for drawing to appear
/// }
/// ```
pub struct Window<'a> {
    window: RenderWindow,
    input_state: InputState,
    render_queue: VecDeque<RenderTask>,
    background_color: Color,
    color_state: ColorState,
    shape_store: ShapeStore<'a>,
    deltatime_clock: Clock,
    deltatime: f32,
    runtime_clock: Clock,
    runtime: f32,
}

impl Window<'_> {
    fn new_from_window(window: RenderWindow) -> Self {
        unsafe {
            if let None = TEXTURES {
                TEXTURES = Some(Vec::new())
            }
        }

        let mut circle_shape = CircleShape::new(0., 32);
        circle_shape.set_outline_thickness(1.);
        let mut rectangle_shape = RectangleShape::new();
        rectangle_shape.set_outline_thickness(1.);
        let texture_shape = RectangleShape::new();

        Self {
            window,
            input_state: InputState::new(),
            render_queue: VecDeque::new(),
            background_color: Color::LIGHT_GRAY,
            color_state: Default::default(),
            shape_store: ShapeStore {
                circle: circle_shape,
                rectangle: rectangle_shape,
                texture: texture_shape,
            },
            runtime_clock: Clock::start(),
            deltatime_clock: Clock::start(),
            deltatime: 1. / 60., // So that we don't get problems in the first frame
            runtime: 0.,
        }
    }

    /// Create a new window of size (`width`, `height`) and with title `name`.
    /// Can be directly drawn to with functions like `.circle(...)`
    /// and has to be updated with `.update()`.
    pub fn new(width: u32, height: u32, name: &str) -> Self {
        let mut window = RenderWindow::new(
            (width, height),
            name,
            Style::TITLEBAR | Style::CLOSE,
            &Default::default(),
        );
        window.set_vertical_sync_enabled(true);
        window.set_key_repeat_enabled(false);

        Self::new_from_window(window)
    }

    /// Create a new fullscreen window.
    /// Can be directly drawn to with functions like `.circle(...)`
    /// and has to be updated with `.update()`.
    pub fn new_fullscreen() -> Self {
        let mut window = RenderWindow::new(
            VideoMode::desktop_mode(),
            "",
            Style::FULLSCREEN,
            &Default::default(),
        );
        window.set_vertical_sync_enabled(true);
        window.set_key_repeat_enabled(false);

        Self::new_from_window(window)
    }

    /// Has to be called every frame for drawings to appear on the screen and keyboard/mouse to be updated.
    /// Note that this function will block for vertical sync.
    pub fn update(&mut self) {
        self.update_events();
        self.update_draw();

        self.deltatime = self.deltatime_clock.restart().as_seconds();
        self.runtime = self.runtime_clock.elapsed_time().as_seconds();

        self.color_state = Default::default();
    }

    fn update_events(&mut self) {
        self.input_state.clear();
        while let Some(event) = self.window.poll_event() {
            self.input_state.handle_event(event);
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => exit(0),
                _ => {}
            }
        }
    }

    fn update_draw(&mut self) {
        self.window.clear(self.background_color.into());
        for task in &self.render_queue {
            let RenderTask {
                pos,
                shape,
                color_state,
            } = task;

            match shape {
                Shapes::Circle { radius } => {
                    let s = &mut self.shape_store.circle;
                    s.set_radius(*radius);
                    s.set_origin((s.radius(), s.radius()));
                    s.set_position(*pos);
                    s.set_fill_color(color_state.fill_color.into());
                    s.set_outline_color(color_state.outline_color.into());
                    self.window.draw(s);
                }
                Shapes::Rectangle { width, height } => {
                    let s = &mut self.shape_store.rectangle;
                    s.set_size((*width, *height));
                    // s.set_origin((*width / 2., *height / 2.));
                    s.set_position(*pos);
                    s.set_fill_color(color_state.fill_color.into());
                    s.set_outline_color(color_state.outline_color.into());
                    self.window.draw(s);
                }
                Shapes::Texture {
                    index,
                    width,
                    height,
                } => unsafe {
                    if let Some(textures) = &TEXTURES {
                        let s = &mut self.shape_store.texture;
                        let tex = &textures[*index];
                        s.set_texture(tex, false);
                        s.set_size((*width, *height));
                        // s.set_origin((*width / 2., *height / 2.));
                        s.set_position(*pos);
                        self.window.draw(s);
                    }
                },
            }
        }
        self.render_queue.clear();
        self.window.display();
    }

    /// Set the background color of the window.
    /// The background color does _not_ reset at the beginning of a new frame.
    pub fn background_color<C: Into<Color>>(&mut self, color: C) {
        self.background_color = color.into();
    }

    /// Set the fill color for drawing shapes like `.circle(...)`.
    /// The fill color is reset at the beginning of a new frame to a default value of `Color::WHITE`.
    pub fn fill_color<C: Into<Color>>(&mut self, color: C) {
        self.color_state.fill_color = color.into();
    }

    /// Set the outline color for drawing shapes like `.circle(...)`.
    /// The outline color is reset at the beginning of a new frame to a default value of `Color::TRANSPARENT`.
    pub fn outline_color<C: Into<Color>>(&mut self, color: C) {
        self.color_state.outline_color = color.into();
    }

    /// Draw a circle at position `pos` with radius `radius`.
    /// The origin of the circle is at it's center.
    pub fn circle(&mut self, pos: (f32, f32), radius: f32) {
        self.render_queue.push_back(RenderTask {
            pos,
            shape: Shapes::Circle { radius },
            color_state: self.color_state.clone(),
        })
    }

    /// Draw a rectangle at position `pos` with width and height of `(width, height)`.
    /// The origin of the rectangle is at it's top left.
    pub fn rectangle(&mut self, pos: (f32, f32), width: f32, height: f32) {
        self.render_queue.push_back(RenderTask {
            pos,
            shape: Shapes::Rectangle { width, height },
            color_state: self.color_state.clone(),
        })
    }

    /// Draw a square at position `pos` with a width and height of `size`.
    /// The origin of the square is at it's top left.
    pub fn square(&mut self, pos: (f32, f32), size: f32) {
        self.render_queue.push_back(RenderTask {
            pos,
            shape: Shapes::Rectangle {
                width: size,
                height: size,
            },
            color_state: self.color_state.clone(),
        })
    }

    /// Draw a texture `texture` at position `pos` with width and height of `(width, height)`.
    /// The origin of the texture is at it's top left.
    /// Textures can be loaded with `.load_texture(...)`.
    /// # Examples
    /// ```
    /// let mut pg = Window::new_fullscreen();
    /// let my_texture = pg.load_texture("my_texture.png").unwrap();
    /// loop {
    ///     pg.texture((100., 250.), my_texture, 100., 150.);
    ///
    ///     pg.update();
    /// }
    /// ```
    pub fn texture(
        &mut self,
        pos: (f32, f32),
        texture: Texture,
        width: f32,
        height: f32,
    ) {
        self.render_queue.push_back(RenderTask {
            pos,
            shape: Shapes::Texture {
                index: texture.index,
                width: width,
                height: height,
            },
            color_state: self.color_state.clone(),
        })
    }

    /// Draw a texture `texture` at position `pos` with width of `width`, with the height according to the aspect ratio of the texture.
    /// The origin of the texture is at it's top left.
    /// Textures can be loaded with `.load_texture(...)`.
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
    pub fn texture_(&mut self, pos: (f32, f32), texture: Texture, width: f32) {
        self.render_queue.push_back(RenderTask {
            pos,
            shape: Shapes::Texture {
                index: texture.index,
                width: width,
                height: width / texture.aspect(),
            },
            color_state: self.color_state.clone(),
        })
    }

    /// Whether the keyboard key `key` is currently held pressed.
    /// # Examples
    /// ```
    /// if pg.key_pressed(Key::SPACE) {
    ///     /*...*/
    /// }
    /// ```
    pub fn key_pressed(&self, key: Key) -> bool {
        self.input_state.key_pressed(key)
    }

    /// Whether they keyboard key `key` has just been pressed in this frame.
    /// # Examples
    /// ```
    /// if pg.key_just_pressed(Key::SPACE) {
    ///     /*...*/
    /// }
    /// ```
    pub fn key_just_pressed(&self, key: Key) -> bool {
        self.input_state.key_just_pressed(key)
    }

    /// Whether the mouse button `button` is currently held pressed.
    /// # Examples
    /// ```
    /// if pg.mouse_pressed(Button::LEFT) {
    ///     /*...*/
    /// }
    /// ```
    pub fn mouse_pressed(&self, button: Button) -> bool {
        self.input_state.mouse_pressed(button)
    }

    /// Whether the mouse button `button`  has just been pressed in this frame.
    /// # Examples
    /// ```
    /// if pg.mouse_just_pressed(Button::LEFT) {
    ///     /*...*/
    /// }
    /// ```
    pub fn mouse_just_pressed(&self, button: Button) -> bool {
        self.input_state.mouse_just_pressed(button)
    }

    /// The current mouse position inside the window.
    pub fn mouse_position(&self) -> (f32, f32) {
        self.input_state.mouse_position()
    }

    /// The current cumulative scroll wheel state of the mouse.
    pub fn mouse_wheel(&self) -> f32 {
        self.input_state.mouse_wheel()
    }

    /// How much the scroll wheel has been scrolled in this frame.
    pub fn mouse_wheel_delta(&self) -> f32 {
        self.input_state.mouse_wheel_delta()
    }

    /// The width of the window, or the width of the screen in fullscreen mode.
    pub fn width(&self) -> f32 {
        self.window.size().x as f32
    }

    /// The height of the window, or the height of the screen in fullscreen mode.
    pub fn height(&self) -> f32 {
        self.window.size().y as f32
    }

    /// The time since the window has been created in seconds.
    pub fn time(&self) -> f32 {
        self.runtime
    }

    /// How much time has passed since the last frame.
    pub fn deltatime(&self) -> f32 {
        self.deltatime
    }

    /// Load a texture from path `path`.
    /// A return value of `None` means that the texture could not be loaded.
    /// On success, returns a `Texture` object that can be passed to the `.texture(...)` function to draw the texture to the screen.
    /// 
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
    pub fn load_texture(&mut self, path: &str) -> Option<Texture> {
        unsafe {
            if let Some(textures) = &mut TEXTURES {
                textures.push(SfmlTexture::from_file(path)?);
                Some(Texture {
                    index: textures.len() - 1,
                })
            } else {
                None
            }
        }
    }
}

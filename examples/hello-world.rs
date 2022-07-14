//! A demonstrations of various features available in the Pronto Graphics library.
//! Please read the comments above each line for an explanation, and the full documentation
//! of the various functions for further details.

use pronto_graphics::{Button, Color, Key, Window};

fn main() {
    // Create a new Pronto Graphics window, all drawing happens through this object.
    // The window's drawable area will be of size 800 by 600, and the window title will be "Hello World".
    let mut pg = Window::new(800, 600, "Hello World");

    // Alternatively, we can create a full screen window.
    // let mut pg = Window::new_fullscreen();

    // Set the background color for our window.
    // At the beginning of each frame, the window is cleared with this color.
    pg.background_color((0x11, 0x11, 0x11));

    // Load a texture from a file.
    let texture = pg.load_texture("examples/test_texture.png").unwrap();

    // Load a font from a file.
    let font = pg.load_font("examples/Whisper-Regular.ttf").unwrap();

    loop {
        // Set the fill color for drawing shapes to blue.
        pg.fill_color(Color::BLUE);
        // Draw a circle in the middle of the window (width/2, height/2) with a radius of 64 pixels.
        pg.circle((pg.width() / 2., pg.height() / 2.), 64.);

        {
            let pos = (
                0.5 * pg.width() + 128. * pg.time().cos(),
                0.5 * pg.height() + 128. * pg.time().sin(),
            );

            // Set the fill color to #CCCCCC, i.e. a light grey.
            pg.fill_color((0xCC, 0xCC, 0xCC));
            // Draw a circle at position `pos` with a radius of 16 pixels.
            pg.circle(pos, 16.);
        }

        // Draw our texture at position (10, 10) with a width of 150 pixels.
        // The texture's height will be deduced from the texture's aspect ratio.
        // I.e. the texture is not distorted.
        // If we want to set the height ourselves, we have to use `pg.texture(...)` (Note the missing `_`)
        pg.texture_((10., 10.), texture, 150.);

        // React to the the left mouse button being pressed.
        // As long as the button is held down, the code inside the `if` is executed.
        if pg.mouse_pressed(Button::LEFT) {
            // Set the fill color to blue, but with it's alpha set to 127, i.e. half-transparent.
            pg.fill_color(Color::GREEN.with_alpha(127));
            pg.circle(pg.mouse_position(), 16.);
        }

        // React to the `SPACE` key having just been pressed this very frame.
        // The code inside the if will only be executed once every time you press the key.
        // I.e. Holding the key down will _not_ cause "Action!" to be printed every frame.
        // If we want to react to a held down key, we have to use `pg.key_pressed(...)`
        if pg.key_just_pressed(Key::SPACE) {
            println!("Action!");
        }

        // React to the mouse wheel having been scrolled.
        // `pg.mouse_wheel()` will give you a cumulative amount of scrolling across frames.
        // If you want to know whether the scroll wheel has been scrolled in the current frame,
        // use `pg.mouse_wheel_delta()` instead.
        if pg.mouse_wheel() > 0. {
            pg.fill_color(Color::GREEN);
        } else {
            pg.fill_color(Color::RED);
        }

        pg.font_color(Color::GREEN);
        pg.font_size(40);
        pg.font(Some(font));
        pg.text((300., 50.), "Salutations!");

        // Draw a square with side lengths of 25 pixels.
        // Depending on what fill color has last been set above,
        // this square will either be green or red.
        pg.square((50., 300.), 25.);

        {
            let pos = (
                pg.width() - 200.,
                50. + ((50. * pg.time()) % (pg.height() - 130.)),
            );

            // Set the font color to a lovely pink of #EE4488.
            pg.font_color((0xEE, 0x44, 0x88));
            // Set the font size to 30 pixels.
            pg.font_size(30);
            // Set the font to the default font.
            pg.font(None);
            // Draw the string "Greetings!" as position `pos`.
            pg.text(pos, "Greetings!");
        }

        // Set the line color from HSB, with the hue rotating through the colors.
        pg.line_color(Color::from_hsb((60. * pg.time()) % 360., 1., 1.));

        // Draw a bunch of lines.
        pg.line((50., 500.), (50., 450.));
        pg.line((50., 450.), (100., 500.));
        pg.line((100., 475. + 25. * (5. * pg.time()).sin()), (150., 450.));

        // At the end of our frame, we have to update the window
        // for our drawings to appear on the screen and for the keyboard/mouse
        // states to be updated.
        // If your window ever is blank or looks glitchy, make sure you haven't forgotten
        // to call `pg.update()` at the end of your `loop`.
        pg.update();
    }
}

use pronto_graphics::{Button, Color, Key, Window};

fn main() {
    let mut pg = Window::new(800, 600, "Hello World");
    pg.background_color((0x11, 0x11, 0x11));
    // let mut pg = Window::new_fullscreen();

    let texture = pg.load_texture("examples/test_texture.png").unwrap();

    loop {
        if pg.key_just_pressed(Key::SPACE) {
            println!("Action!");
        }

        pg.fill_color((0x1D, 0x37, 0x85));
        pg.circle((pg.width() / 2., pg.height() / 2.), 64.);

        let pos = (
            0.5 * pg.width() + 128. * pg.time().cos(),
            0.5 * pg.height() + 128. * pg.time().sin(),
        );
        pg.fill_color((0xCC, 0xCC, 0xCC));
        pg.circle(pos, 16.);

        pg.texture_((10., 10.), texture, 100.);

        if pg.mouse_pressed(Button::LEFT) {
            pg.fill_color(Color::BLUE);
            pg.circle(pg.mouse_position(), 16.);
        }

        pg.update();
    }
}

# Pronto Graphics

You want some circles on the screen?

```rust
let mut pg = Window::new(800, 600, "Circles!");
loop {
    pg.circle((100., 100.), 15.);

    pg.fill_color(Color::BLUE);
    pg.circle((300., 450.), 24.);
    
    pg.update();
}
```

Some rectangles?

```rust
let mut pg = Window::new(800, 600, "Rectangles!");
pg.background_color(Color::DARK_GRAY);

loop {
    pg.fill_color(Color::RED);
    pg.rectangle((50., 100.), 80., 60.);

    pg.outline_color(Color::BLACK);
    pg.fill_color((0x1D, 0x37, 0x85));
    pg.rectangle((300., 450.), 100., 120.);

    pg.update();
}
```

Or a texture?

```rust
let mut pg = Window::new_fullscreen();
let my_texture = pg.load_texture("my_texture.png").unwrap();
loop {
    pg.texture((100., 250.), my_texture, 100., 150.);
    
    pg.update();
}
```

Want to press some keys and click some places?

```rust
let mut pg = Window::new_fullscreen();
loop {
    if pg.key_just_pressed(Key::SPACE) {
            println!("Action!");
    }

    if pg.mouse_pressed(Button::RIGHT) {
        pg.circle(pg.mouse_position(), 16.);
    }

    pg.update();
}
```

A library for anyone who just wants to get some graphics on the screen, without having to faff about with the technical details of graphics libraries. Just create a window, and start drawing to it.

Loosely inspired by [Processing](https://processing.org/).

## Features

- [X] Circle, Rectangles, Squares
- [ ] Lines
- [X] Textures
- [X] Keyboard
- [X] Mouse
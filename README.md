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

Or a texture?

```rust
let mut pg = Window::new_fullscreen();
let my_texture = pg.load_texture("my_texture.png").unwrap();
loop {
    pg.texture((100., 250.), my_texture, 100., 150.);
    
    pg.update();
}
```

How about some text?

```rust
let mut pg = Window::new(720, 480, "Text!");
loop {
    pg.font_color((0xEE, 0x44, 0x88));
    pg.font_size(30);
    pg.text((100., 100.), "Greetings!");
    
    pg.update();
}
```

Want to press some keys and click some buttons?

```rust
let mut pg = Window::new_fullscreen();
loop {
    if pg.key_just_pressed(Key::SPACE) {
        println!("Action!");
    }

    if pg.mouse_pressed(Button::LEFT) {
        pg.circle(pg.mouse_position(), 16.);
    }

    pg.update();
}
```

## Premise

A library for anyone who just wants to get some graphics on the screen, without having to faff about with the technical details of proper graphics libraries. Just create a window, and start drawing to it. Minimal setup, minimal bother, minimal interference with your own code's structure.

Loosely inspired by [Processing](https://processing.org/).

## Features

- [X] Circle, Rectangles, Squares
- [X] Textures
- [X] Text
- [X] Keyboard
- [X] Mouse
- [X] Doc comments

### Planned

- [ ] Lines
- [ ] Fonts

## Usage

Since the library currently still is in an early stage, I haven't published it to [crate.io](https://crates.io/) yet, but thanks to `cargo` being awesome, you can simply include it in your project like this for now:

```toml
[dependencies]
pronto-graphics = {git = "https://github.com/Garbaz/pronto-graphics"}
```

## Todos

A few ideas of things that could be implemented or improved

- 
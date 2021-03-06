# Pronto Graphics

|| [__Docs.rs__](https://docs.rs/pronto-graphics/latest/pronto_graphics/) || [__Lib.rs__](https://lib.rs/crates/pronto-graphics) || [__Crates.io__](https://crates.io/crates/pronto-graphics/) ||

A library for anyone who just wants to get some graphics on the screen, without having to faff about with the technical details of proper graphics libraries. Just create a window, and start drawing to it. Minimal setup, minimal bother, minimal interference with your own code's structure.

Loosely inspired by [Processing](https://processing.org/).

## Examples

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

Want to press some keys and click some stuff?

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

## Install

Since Pronto Graphics uses [SFML](https://www.sfml-dev.org/) for rendering, specifically the [SFML bindings for Rust](https://docs.rs/sfml/latest/sfml/index.html), it's prerequisites are [identical to the SFML Rust bindings](https://docs.rs/sfml/latest/sfml/index.html#prerequisites), namely for the [SFML library](https://www.sfml-dev.org/) to be installed. This might possibly change in future versions.

```toml
# Cargo.toml

[dependencies]
pronto-graphics = "0.4.0"
```

## Features

- [X] Circle, Rectangles, Squares
- [X] Lines
- [X] Textures
- [X] Text
- [X] Keyboard
- [X] Mouse
- [X] Doc comments
- [X] Custom fonts

### Planned

- [ ] Transforms (Rotation, Origin, text alignment)
- [ ] Triangles
- [ ] Arbitrary shapes
- [ ] Audio
- [ ] Caching/Batching of draw calls
- [ ] Draw customization (outline thickness/etc.)

## Contributions

I'm no experiences open saucer either, so don't hesitate with pull requests :)

Just make sure to explain what you're adding or changing and to adequately add [doc comments](https://doc.rust-lang.org/reference/comments.html#doc-comments) to anything that will be exposed to the end-user.

## Notes

- Investigate whether it might be better for performance to create a new SFML object for each draw call, instead of reusing them.
- Add separate examples for different features, the `hello-world.rs` is getting quite big.

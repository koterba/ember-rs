# Ember

Ember is a simple and fun 2D rendering library for Rust, allowing you to quickly create graphics and interactive applications with ease. It uses the `minifb` crate under the hood to handle window creation and input events.

## Features

- Easy-to-use API for drawing basic shapes, such as circles, lines, and rectangles
- Support for drawing text with a built-in font
- Efficient pixel buffer manipulation
- Mouse and keyboard input handling

## Examples

The following example demonstrates some of Ember's features:

```rust
use ember_rs::Ember;

fn main() {
    let (width, height, fps) = (800, 600, 144.0);
    let mut app = Ember::new("Ember - Simple Example", width, height, fps);

    while !app.should_close() {
        // clear the previous drawing
        app.clear();

        // draw text at the top-left of the screen at size 4
        app.draw_text("ember is awesome", 20, 20, 4, 0xFFAAAA);

        // draw a filled white circle at the bottom-right of the window
        app.draw_circle_fill(700, 500, 40, 0xFFFFFF);

        // draw a green-ish line going from the text to the circle
        app.draw_line(310, 70, 630, 430, 0xAAFFAA);

        // update the screen with the new drawing
        app.update();
    }
}
```

## Installation

To get started with Ember, type the following command inside of your project:

```
cargo add ember-rs
```

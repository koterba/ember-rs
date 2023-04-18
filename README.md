# Ember

Ember is a simple and fun 2D rendering library for Rust, allowing you to quickly create graphics and interactive applications with ease. It uses the `minifb` crate under the hood to handle window creation and input events.

## Features

- Easy-to-use API for drawing basic shapes, such as circles, lines, and rectangles
- Support for drawing text with a built-in font
- Efficient pixel buffer manipulation
- Mouse and keyboard input handling

## Example

The following example demonstrates some of Ember's features:

```rust
use ember::Ember;

fn main() {
    let (width, height) = (800, 800);
    let fps = 60.0;
    let mut screen = Ember::new("Ember - Simple Example", width, height, fps);

    while !screen.should_close() {
        screen.clear();

        // Draw a red circle at the center of the window
        screen.draw_circle(width / 2, height / 2, 50, 0xFF0000);

        // Draw a green rectangle at the top-left corner
        screen.draw_rectangle_fill(20, 20, 120, 70, 0x00FF00);

        // Draw light blue text at the bottom-left corner at size 3
        screen.draw_text("Ember is awesome!", 20, 540, 3, 0x22AAFF);

        // Update the window with the new drawing
        screen.update();
    }
}
```

## Installation

To get started with Ember, type the following command inside of your project:

```
cargo add ember-rs
```

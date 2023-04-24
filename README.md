<div align="center">
  <img width="60%" src="https://cdn.discordapp.com/attachments/947092663914623016/1099661929028522027/ember_logo.png" />

# Ember

Ember is a simple and fun 2D rendering library for Rust, allowing you to quickly create graphics and interactive applications with ease. It uses the `minifb` crate under the hood to handle window creation and input events.

</div>

## Features

- Easy-to-use API for drawing basic shapes, such as circles, lines, and rectangles
- Support for drawing text with a built-in font
- Efficient pixel buffer manipulation
- Mouse and keyboard input handling

## Examples

### Basic Drawing Example

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

### Mouse and Keyboard Input Example

The following example demonstrates mouse and keyboard input handling:

```rust
use ember_rs::{Ember, Key};

fn main() {
    let (width, height, fps) = (800, 600, 144.0);
    let mut app = Ember::new("Ember - Mouse and Keyboard Example", width, height, fps);

    let mut circle_x = width / 2;
    let mut circle_y = height / 2;
    let circle_radius = 40;

    while !app.should_close() {
        // clear the previous drawing
        app.clear();

        // handle keyboard input
        app.process_keys(|key| match key {
            Key::W => circle_y -= 5,
            Key::A => circle_x -= 5,
            Key::S => circle_y += 5,
            Key::D => circle_x += 5,
            _ => {}
        });

        // alternatively use app.get_keys() which returns a Vec<Key>

        // handle mouse input
        let mouse_info = app.get_mouse_info();
        if let Some((mouse_x, mouse_y)) = mouse_info.position {
            if mouse_info.left_button {
                circle_x = mouse_x as i32;
                circle_y = mouse_y as i32;
            }
        }

        // draw a filled circle at the current position
        app.draw_circle_fill(circle_x, circle_y, circle_radius, 0xFF0000);

        // update the screen with the new drawing
        app.update();
    }
}
```

### Filled Shapes Example

The following example demonstrates drawing various filled shapes:

```rust
// this method is used under the hood for all the other methods
app.set_pixel(x, y, color);

// draw a line
app.draw_line(x1, y1, x2, y2, color);

// draw a line with a specific width
app.draw_line_width(x1, y1, x2, y2, width, color);

// draw a rectangle 
app.draw_rectangle(x, y, width, height, color);

// draw a filled rectangle
app.draw_rectangle_fill(x, y, width, height, color);

// draw a circle
app.draw_circle(x, y, radius, color);

// draw a filled circle
app.draw_circle_fill(x, y, radius, color);
```

### Draw Text Example

The following example demonstrates how to draw text to the screen:

```rust
use ember_rs::{Ember, helper};

fn main() {
    let (width, height, fps) = (800, 600, 144.0);
    let mut app = Ember::new("Ember - Mouse and Keyboard Example", width, height, fps);

    while !app.should_close() {
        // clear the previous drawing
        app.clear();

        // draw text to the screen at position (20, 20), scaled 4 times
        app.draw_text("hello world", 50, 50, 4, 0xFFFFFF);

        // if you want to center the text, use one of the helper methods
        let text = format!("Width: {}", width);
        // this return the (x, y) needed to center the text
        let (x, y) = helper::center_text(&text, 400, 400, 4);
        // draw the centered text
        app.draw_text(&text, x, y, 4, 0xAAAAFF);

        // update the screen with the new drawing
        app.update();
    }
}
```

### Color Example

The following example demonstrates ways to create colors:

```rust
use ember_rs::Color

// shortest way to write a color
let color = 0xFFFFFF;

// from RGB
let color = Color::from_rgb(255, 255, 255);

// from HSL
let color = Color::from_hsl(0.0, 0.0, 100.0);

// from hex string - results to FFFFFF if unable to parse
let color = Color::from_hex("FFFFFF");
```

## Installation

To get started with Ember, type the following command inside of your project:

```
cargo add ember-rs
```

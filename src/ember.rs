use minifb::{Window, WindowOptions, MouseMode, MouseButton};
pub use minifb::Key;

use std::collections::HashMap;

use crate::text::{self, FONT_HEIGHT};

pub struct Ember {
    pub width: i32,
    pub height: i32,

    buffer: Vec<u32>,
    font: HashMap<char, [u8; FONT_HEIGHT]>,

    window: Window
}

pub struct MouseInfo {
    pub position: Option<(f32, f32)>,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    pub wheel: Option<(f32, f32)>
}

impl Ember {
    pub fn new(title: &str, width: i32, height: i32, fps: f32) -> Self {
        let mut window = Window::new(
            title,
            width as usize,
            height as usize,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        if fps > 0.0 {
            let fps: f32 = 1.0 / (fps / 1_000_000.0);
            let fps: u64 = fps as u64;

            window.limit_update_rate(Some(std::time::Duration::from_micros(fps)));
        }

        let buffer: Vec<u32> = vec![0; (width * height) as usize];

        let font = text::get_font();

        Self {  width, height, buffer, font, window }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer[..], self.width as usize, self.height as usize)
            .unwrap();
    }

    // Using the 'checked_...' methods for safer arithmetic operations.
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if let Some(index) = (y as usize).checked_mul(self.width as usize).and_then(|i| i.checked_add(x as usize)) {
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;

        let mut x = x1;
        let mut y = y1;
        loop {
            self.set_pixel(x, y, color);
            if x == x2 && y == y2 {
                break;
            }
            err2 = err;
            if err2 > -dx {
                err -= dy;
                x += sx;
            }
            if err2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_line_width(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, width: i32, color: u32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        let half_width = width / 2;

        while x != x1 || y != y1 {
            // Draw a square around the point (x, y) with a side length of `width`
            for i in -half_width..=half_width {
                for j in -half_width..=half_width {
                    self.set_pixel(
                        x + i,
                        y + j,
                        color
                    );
                }
            }

            let err2 = 2 * err;

            if err2 > -dy {
                err -= dy;
                x += sx;
            }
            if err2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        let x2 = x + width;
        let y2 = y + height;

        self.draw_line(x, y, x2, y, color);
        self.draw_line(x2, y, x2, y2, color);
        self.draw_line(x2, y2, x, y2, color);
        self.draw_line(x, y2, x, y, color);
    }

    pub fn draw_rectangle_fill(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for row in y..y + height {
            self.draw_line(x, row, x + width, row, color);
        }
    }

    pub fn draw_circle(&mut self, x0: i32, y0: i32, radius: i32, color: u32) {
        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            self.set_pixel(x0 + x, y0 + y, color);
            self.set_pixel(x0 + y, y0 + x, color);
            self.set_pixel(x0 - y, y0 + x, color);
            self.set_pixel(x0 - x, y0 + y, color);
            self.set_pixel(x0 - x, y0 - y, color);
            self.set_pixel(x0 - y, y0 - x, color);
            self.set_pixel(x0 + y, y0 - x, color);
            self.set_pixel(x0 + x, y0 - y, color);

            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }

    pub fn draw_circle_fill(&mut self, x0: i32, y0: i32, radius: i32, color: u32) {
        let mut x = 0;
        let mut y = radius;
        let mut dp = 1 - radius;

        while x <= y {
            self.draw_line(x0 - x, y0 - y, x0 + x, y0 - y, color);
            self.draw_line(x0 - x, y0 + y, x0 + x, y0 + y, color);
            self.draw_line(x0 - y, y0 - x, x0 + y, y0 - x, color);
            self.draw_line(x0 - y, y0 + x, x0 + y, y0 + x, color);

            if dp < 0 {
                dp = dp + 2 * x + 3;
            } else {
                dp = dp + 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, scale: i32, color: u32) {
        let mut current_x = x;

        for ch in text.to_uppercase().chars() {

            let character = self.font.get(&ch).unwrap_or(&text::EMPTY).clone();

            for i in 0..8 {
                let row = character[i];
                for j in 0..8 {
                    if row & (1 << (7 - j)) != 0 {
                        for s_y in 0..scale {
                            for s_x in 0..scale {
                                self.set_pixel(current_x + j * scale + s_x, y + i as i32 * scale + s_y, color);
                            }
                        }
                    }
                }
            }

            current_x += 8 * scale;
        }
    }

    pub fn get_keys(&mut self) -> Vec<Key> {
        self.window.get_keys()
    }

    pub fn process_keys<F>(&mut self, f: F)
    where
        F: FnMut(&Key)
    {
        self.window.get_keys().iter().for_each(f);
    }

    pub fn get_mouse_info(&self) -> MouseInfo {
        let position = self.window.get_mouse_pos(MouseMode::Clamp).map(|(x, y)| (x as f32, y as f32));
        let left_button = self.window.get_mouse_down(MouseButton::Left);
        let right_button = self.window.get_mouse_down(MouseButton::Right);
        let middle_button = self.window.get_mouse_down(MouseButton::Middle);
        let wheel = self.window.get_scroll_wheel().map(|(x, y)| (x as f32, y as f32));

        MouseInfo {
            position,
            left_button,
            right_button,
            middle_button,
            wheel
        }
    }

    pub fn should_close(&mut self) -> bool {
        !self.window.is_open() || self.window.is_key_down(Key::Escape)
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn clear_color(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }
}

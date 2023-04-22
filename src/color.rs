pub struct Color;

impl Color {
    pub fn from_hex(hex: &str) -> u32 {
        u32::from_str_radix(hex, 16).unwrap_or(0xFFFFFF)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> u32 {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
    
        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
    
        ((r + m) * 255.0) as u32 * 0x10000 + ((g + m) * 255.0) as u32 * 0x100 + ((b + m) * 255.0) as u32
    }
}

pub struct Color;

impl Color {
    pub fn from_hex(hex: &str) -> u32 {
        u32::from_str_radix(hex, 16).unwrap_or(0xFFFFFF)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
}

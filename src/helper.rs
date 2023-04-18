pub fn center_text(text: &str, x: i32, y: i32, scale: i32) -> (i32, i32, i32) {
    let text_length = text.len() as i32;
    let width  = (text_length * 8) + scale;
    let height = 8 + scale;

    let x = x-(width/2);
    let y = y-(height/2);

    (x, y, scale)
}

pub fn at_angle(x1: i32, y1: i32, length: i32, angle: f32) -> (i32, i32) {
    let angle = angle / 57.29577951;
    let x2 = x1 as f32 + (angle.sin() * length as f32);
    let y2 = y1 as f32 + (angle.cos() * length as f32);

    (x2 as i32, y2 as i32)
}
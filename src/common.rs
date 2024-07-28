#[derive(Clone, Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA {
            r,
            g,
            b,
            a,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: RGBA,
}

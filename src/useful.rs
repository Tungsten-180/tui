pub const ESC: &str = "\u{001b}";

#[derive(Clone, Copy, Debug)]
pub enum Alignment {
    Centered,
    None,
}
#[derive(Clone, Copy, Debug)]
pub struct Dimension {
    pub height: u32,
    pub width: u32,
}
impl Default for Dimension {
    fn default() -> Self {
        Self {
            height: 1,
            width: 0,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Coordinate(pub i32, pub i32);
impl Coordinate {
    pub fn as_array(&self) -> [i32; 2] {
        [self.0, self.1]
    }
}

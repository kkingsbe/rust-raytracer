pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}
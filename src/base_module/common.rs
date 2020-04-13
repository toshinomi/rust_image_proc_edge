pub enum PixelKind {
    Red,
    Green,
    Blue,
    Alpha,
} 

pub struct Pixel<T> {
    red:    T,
    green:  T,
    blue:   T,
    alpha:  T,
}

impl<T> Pixel<T> {
    pub fn new(red_value: T, green_value: T, blue_value: T, alpha_value: T) -> Pixel<T> {
        Pixel { red: red_value, green: green_value, blue: blue_value, alpha: alpha_value }
    }
    pub fn set_red(&mut self, value: T) {
        self.red = value;
    }
    pub fn get_red(&mut self) -> &T {
        &self.red
    }
    pub fn set_green(&mut self, value: T) {
        self.green = value;
    }
    pub fn get_green(&mut self) -> &T {
        &self.green
    }
    pub fn set_blue(&mut self, value: T) {
        self.blue = value;
    }
    pub fn get_blue(&mut self) -> &T {
        &self.blue
    }
    pub fn set_alpha(&mut self, value: T) {
        self.alpha = value;
    }
    pub fn get_alpha(&mut self) -> &T {
        &self.alpha
    }
}

pub trait PixelSumDataToU8 {
    fn to_u8(self) -> u8;
}

impl PixelSumDataToU8 for i64 {
    fn to_u8(self) -> u8 {
        let cnv_value: u8;
        if self > 255i64 {
            cnv_value = 255u8;
        }
        else if self < 0 {
            cnv_value = 0u8;
        }
        else {
            cnv_value = self as u8;
        }
        cnv_value
    }
}
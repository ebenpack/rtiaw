use std::ops::Add;
use std::ops::Mul;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn hex_red(&self) -> i32 {
        (self.red * 255.0).round() as i32
    }

    pub fn hex_green(&self) -> i32 {
        (self.green * 255.0).round() as i32
    }

    pub fn hex_blue(&self) -> i32 {
        (self.blue * 255.0).round() as i32
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            red: self * rhs.red,
            green: self * rhs.green,
            blue: self * rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * &self
    }
}

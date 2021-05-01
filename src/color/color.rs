use crate::vec3::Vec3;
use rand::Rng;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn hex_red(&self) -> i32 {
        (self.red * 255.0).round() as i32
    }

    pub fn hex_green(&self) -> i32 {
        (self.green * 255.0).round() as i32
    }

    pub fn hex_blue(&self) -> i32 {
        (self.blue * 255.0).round() as i32
    }

    pub fn random(min: f64, max: f64) -> Color {
        let red = rand::thread_rng().gen_range(min..max);
        let green = rand::thread_rng().gen_range(min..max);
        let blue = rand::thread_rng().gen_range(min..max);
        Color::new(red, green, blue)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red = self.red + rhs.red;
        self.green = self.green + rhs.green;
        self.blue = self.blue + rhs.blue;
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

impl Add<Vec3> for Color {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Color {
            red: self.red + rhs.x,
            green: self.green + rhs.y,
            blue: self.blue + rhs.z,
        }
    }
}

impl Add<Color> for Vec3 {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        rhs + self
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.red = self.red * rhs;
        self.green = self.green * rhs;
        self.blue = self.blue * rhs;
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.red = self.red * rhs.red;
        self.green = self.green * rhs.green;
        self.blue = self.blue * rhs.blue;
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

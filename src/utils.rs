use std::ops::Mul;

use image::{Pixel, Rgba};

use crate::vector::Vector3;

#[derive(Debug, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(self.red, self.green, self.blue, 255)
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(255),
            green: self.green.min(255),
            blue: self.blue.min(255),
        }
    }

    fn to_vector(&self) -> Vector3 {
        Vector3 {
            x: self.red as f32 / 255.0,
            y: self.green as f32 / 255.0,
            z: self.blue as f32 / 255.0,
        }
    }

    pub fn to_perc(&self) -> Vector3 {
        Vector3 {
            x: self.red as f32 / 255.0,
            y: self.green as f32 / 255.0,
            z: self.blue as f32 / 255.0,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            red: (self.red as f32 * rhs) as u8,
            green: (self.green as f32 * rhs) as u8,
            blue: (self.blue as f32 * rhs) as u8,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: (self.red as u16 * rhs.red as u16).min(255) as u8,
            green: (self.green as u16 * rhs.green as u16).min(255) as u8,
            blue: (self.blue as u16 * rhs.blue as u16).min(255) as u8,
        }
    }
}

use std::ops::{Add, Mul};

use image::{GenericImageView, Pixel, Rgba};

use crate::vector::Vector2;

#[derive(Debug, Clone)]
pub struct Material {
    pub coloring: Coloring,
    pub albedo: f32,
    pub surface_kind: SurfaceKind,
}

#[derive(Debug, Clone)]
pub enum SurfaceKind {
    Diffuse,
    Reflective { reflectivity: f32 },
    _Refractive, // TODO:
}

impl Material {
    pub fn new(coloring: Coloring, albedo: f32, surface_kind: SurfaceKind) -> Self {
        Self {
            coloring,
            albedo,
            surface_kind,
        }
    }

    pub fn color(&self, coords: Vector2) -> Color {
        match &self.coloring {
            Coloring::Color(color) => *color,
            Coloring::Texture(texture) => texture.color_at(coords),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Coloring {
    Color(Color),
    Texture(Texture),
}

#[derive(Debug, Clone)]
pub struct Texture {
    image: image::DynamicImage,
}

impl Texture {
    pub fn new(image: image::DynamicImage) -> Self {
        Self { image }
    }

    pub fn color_at(&self, coords: Vector2) -> Color {
        let texture_x = self.wrap(coords.x, self.image.width() as f32);
        let texture_y = self.wrap(coords.y, self.image.height() as f32);

        Color::from(self.image.get_pixel(texture_x, texture_y).to_rgba())
    }

    fn wrap(&self, value: f32, limit: f32) -> u32 {
        let coord = (value * limit) % limit;
        if coord < 0.0 {
            (coord + limit) as u32
        } else {
            coord as u32
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(self.red, self.green, self.blue, 255)
    }

    pub fn _clamp(&self) -> Color {
        Color {
            red: self.red.min(255),
            green: self.green.min(255),
            blue: self.blue.min(255),
        }
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgba: Rgba<u8>) -> Self {
        Color {
            red: rgba.0[0],
            green: rgba.0[1],
            blue: rgba.0[2],
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
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

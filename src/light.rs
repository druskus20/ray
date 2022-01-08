use crate::{color::Color, vector::Vector3};

#[derive(Debug, Clone)]
pub enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
}

impl Light {
    pub fn color(&self) -> Color {
        match self {
            Light::Point(light) => light.color,
            Light::Directional(light) => light.color,
        }
    }

    pub fn intensity(&self, point: Vector3) -> f32 {
        match self {
            Light::Point(light) => {
                let rad2 = (light.position - point).norm();
                light.intensity / (4.0 * std::f32::consts::PI * rad2)
            }
            Light::Directional(light) => light.intensity,
        }
    }

    pub fn direction(&self, point: Vector3) -> Vector3 {
        match self {
            Light::Point(light) => (light.position - point),
            Light::Directional(light) => -light.direction,
        }
        .normalize()
    }

    pub fn distance(&self, point: Vector3) -> f32 {
        match self {
            Light::Point(light) => (light.position - point).length(),
            Light::Directional(_) => std::f32::INFINITY,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Debug, Clone)]
pub struct PointLight {
    pub position: Vector3,
    pub color: Color,
    pub intensity: f32,
}

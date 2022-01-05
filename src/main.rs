#![allow(dead_code)]
#![allow(unused_variables)]

mod color;
mod object;
mod render;
mod vector;

use color::Color;
use image::ImageFormat;
use object::{Object, Sphere};
use render::{Light, Scene};
use vector::Vector3;

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        light: Light {
            direction: Vector3::new(0.0, -1.0, -1.0),
            color: Color::new(51, 255, 51),
            intensity: 20.0,
        },
        objects: vec![Object::Sphere(Sphere {
            center: Vector3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            color: Color::new(255, 255, 255),
            albedo: 0.18,
        })],
    };

    let image = scene.render();

    image
        .save_with_format("image.png", ImageFormat::Png)
        .unwrap();
}

#[test]
fn test_can_render_scene() {
    use image::{DynamicImage, GenericImageView};

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        light: Light {
            direction: Vector3::new(0.0, 0.0, -1.0),
            color: Color::new(255, 255, 255),
            intensity: 1.0,
        },
        objects: vec![Object::Sphere(Sphere {
            center: Vector3::zero(),
            radius: 1.0,
            color: Color::new(200, 20, 50),
            albedo: 1.0,
        })],
    };

    let img: DynamicImage = scene.render();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

#![allow(dead_code)]
#![allow(unused_variables)]

mod color;
mod object;
mod render;
mod vector;

use color::Color;
use image::ImageFormat;
use object::{Object, Plane, Sphere};
use render::{Light, Scene};
use vector::Vector3;

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        light: Light {
            direction: Vector3::new(0.3, -0.8, -0.9),
            color: Color::new(255, 255, 255),
            intensity: 20.0,
        },
        objects: vec![
            // Object::Plane(Plane {
            //     normal: Vector3::new(-0.0, -1.0, -0.0),
            //     color: Color::new(255, 50, 50),
            //     albedo: 0.3,
            //     origin: Vector3::new(0.0, 50.0, -5.0),
            // }),
            // Object::Plane(Plane {
            //     normal: Vector3::new(0.0, 0.0, -1.0),
            //     color: Color::new(50, 90, 200),
            //     albedo: 0.5,
            //     origin: Vector3::new(0.0, 0.0, -9.0),
            // }),
            //Object::Plane(Plane {
            //    normal: Vector3::new(0.0, -1.0, -1.0),
            //    color: Color::new(30, 30, 30),
            //    albedo: 0.2,
            //    origin: Vector3::new(0.0, -3.0, -7.0),
            //}),
            Object::Sphere(Sphere {
                center: Vector3::new(-1.0, 0.0, -3.0),
                radius: 1.0,
                color: Color::new(150, 10, 20),
                albedo: 0.38,
            }),
            Object::Sphere(Sphere {
                center: Vector3::new(1.0, 1.0, -2.0),
                radius: 1.0,
                color: Color::new(40, 10, 200),
                albedo: 0.38,
            }),
            Object::Sphere(Sphere {
                center: Vector3::new(-1.0, 0.0, -8.0),
                radius: 3.0,
                color: Color::new(10, 200, 60),
                albedo: 0.18,
            }),
        ],
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

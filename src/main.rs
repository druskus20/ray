#![allow(dead_code)]
#![allow(unused_variables)]

mod light;
mod material;
mod object;
mod render;
mod vector;

use image::ImageFormat;
use light::{DirectionalLight, Light, PointLight};
use material::{Color, Coloring, Material, Texture};
use object::{Mesh, Object, Plane, Sphere};
use render::Scene;
use vector::Vector3;

fn main() {
    let sample_texture = image::open("sample_texture.png").unwrap();
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: Vector3::new(0.25, -0.5, -0.5),
                color: Color::new(255, 255, 255),
                intensity: 2.0,
            }),
            Light::Point(PointLight {
                position: Vector3::new(0.3, -0.7, -1.5),
                color: Color::new(255, 255, 255),
                intensity: 300.0,
            }),
            Light::Point(PointLight {
                position: Vector3::new(0.0, 10.0, -10.0),
                color: Color::new(255, 255, 255),
                intensity: 10000.0,
            }),
        ],
        objects: vec![
            Object::new(
                //Material::new(Coloring::Color(Color::new(60, 60, 60)), 0.38),
                Material::new(
                    Coloring::Texture(Texture::new(sample_texture.clone())),
                    0.38,
                ),
                Mesh::Plane(Plane {
                    normal: Vector3::new(-0.0, -1.0, -0.0),
                    origin: Vector3::new(0.0, -3.0, 0.0),
                }),
            ),
            Object::new(
                Material::new(Coloring::Color(Color::new(90, 160, 220)), 0.38),
                Mesh::Plane(Plane {
                    normal: Vector3::new(0.0, 0.0, -1.0),
                    origin: Vector3::new(0.0, 0.0, -20.0),
                }),
            ),
            Object::new(
                Material::new(Coloring::Color(Color::new(150, 10, 20)), 0.38),
                Mesh::Sphere(Sphere {
                    center: Vector3::new(-1.0, 0.0, -3.0),
                    radius: 1.0,
                }),
            ),
            Object::new(
                Material::new(
                    Coloring::Texture(Texture::new(sample_texture.clone())),
                    0.38,
                ),
                //Material::new(Coloring::Color(Color::new(40, 10, 200)), 0.38),
                Mesh::Sphere(Sphere {
                    center: Vector3::new(1.0, 1.0, -2.0),
                    radius: 1.0,
                }),
            ),
            Object::new(
                Material::new(Coloring::Color(Color::new(10, 200, 60)), 0.18),
                Mesh::Sphere(Sphere {
                    center: Vector3::new(-1.0, 0.0, -8.0),
                    radius: 3.0,
                }),
            ),
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
        lights: vec![Light::Directional(DirectionalLight {
            direction: Vector3::new(0.0, 0.0, -1.0),
            color: Color::new(255, 255, 255),
            intensity: 1.0,
        })],
        objects: vec![Object {
            material: Material::new(Coloring::Color(Color::new(60, 60, 60)), 0.38),
            mesh: Mesh::Sphere(Sphere {
                center: Vector3::zero(),
                radius: 1.0,
            }),
        }],
    };

    let img: DynamicImage = scene.render();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

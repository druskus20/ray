#![allow(dead_code)]
#![allow(unused_variables)]

mod light;
mod material;
mod object;
mod render;
mod vector;

use image::ImageFormat;
use light::{DirectionalLight, Light, PointLight};
use material::{Color, Coloring, Material, SurfaceKind, Texture};
use object::{Mesh, Object, Plane, Sphere};
use render::Scene;
use vector::Vector3;

fn main() {
    let sample_texture = image::open("sample_texture.png").unwrap();
    let scene = Scene {
        width: 80 * 10,
        height: 60 * 10,
        fov: 90.0,
        max_recursion_depth: 4,
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: Vector3::new(0.45, -0.5, -0.32),
                color: Color::new(120, 250, 120),
                intensity: 2.0,
            }),
            //Light::Point(PointLight {
            //    position: Vector3::new(-1.0, -1.0, 1.0),
            //    color: Color::new(250, 200, 250),
            //    intensity: 800.0,
            //}),

            // Between the spheres
            Light::Point(PointLight {
                position: Vector3::new(-0.1, -0.5, -3.5),
                //color: Color::new(250, 200, 250),
                color: Color::new(120, 175, 120),
                intensity: 160.0,
            }),
            Light::Point(PointLight {
                position: Vector3::new(-0.3, 0.75, -3.2),
                //color: Color::new(250, 200, 250),
                color: Color::new(120, 175, 120),
                intensity: 100.0,
            }),
            // In front of the blue spehere
            Light::Point(PointLight {
                position: Vector3::new(0.3, 0.0, -1.0),
                color: Color::new(120, 175, 120),
                intensity: 50.0,
            }),
            // Top light
            Light::Point(PointLight {
                position: Vector3::new(2.0, 6.0, 2.5),
                color: Color::new(240, 120, 130),
                intensity: 20000.0,
            }),
        ],
        objects: vec![
            Object::new(
                Material::new(
                    Coloring::Texture(Texture::new(sample_texture.clone())),
                    //Coloring::Color(Color::new(160, 160, 160)),
                    0.18,
                    SurfaceKind::Reflective { reflectivity: 0.5 },
                    //SurfaceKind::Diffuse,
                ),
                Mesh::Plane(Plane {
                    normal: Vector3::new(-0.0, -1.0, -0.0),
                    origin: Vector3::new(0.0, -3.0, 0.0),
                }),
            ),
            Object::new(
                Material::new(
                    Coloring::Color(Color::new(90, 160, 220)),
                    0.25,
                    SurfaceKind::Diffuse,
                ),
                Mesh::Plane(Plane {
                    normal: Vector3::new(0.0, 0.0, -1.0),
                    origin: Vector3::new(0.0, 0.0, -12.0),
                }),
            ),
            Object::new(
                Material::new(
                    Coloring::Color(Color::new(150, 10, 20)),
                    0.22,
                    //SurfaceKind::Reflective { reflectivity: 0.75 },
                    SurfaceKind::Diffuse,
                ),
                Mesh::Sphere(Sphere {
                    center: Vector3::new(-1.75, 0.0, -3.0),
                    radius: 1.0,
                }),
            ),
            Object::new(
                Material::new(
                    Coloring::Texture(Texture::new(sample_texture.clone())),
                    0.35,
                    SurfaceKind::Diffuse,
                ),
                //Material::new(Coloring::Color(Color::new(40, 10, 200)), 0.38),
                Mesh::Sphere(Sphere {
                    center: Vector3::new(1.0, 1.0, -2.0),
                    radius: 1.0,
                }),
            ),
            Object::new(
                Material::new(
                    Coloring::Color(Color::new(10, 200, 60)),
                    0.3,
                    SurfaceKind::Reflective { reflectivity: 0.9 },
                    //SurfaceKind::Diffuse,
                ),
                Mesh::Sphere(Sphere {
                    center: Vector3::new(-0.5, 0.75, -6.5),
                    radius: 2.5,
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
        max_recursion_depth: 2,
        lights: vec![Light::Directional(DirectionalLight {
            direction: Vector3::new(0.0, 0.0, -1.0),
            color: Color::new(255, 255, 255),
            intensity: 1.0,
        })],
        objects: vec![Object {
            material: Material::new(
                Coloring::Color(Color::new(60, 60, 60)),
                0.38,
                SurfaceKind::Diffuse,
            ),
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

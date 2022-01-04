#![allow(dead_code)]
#![allow(unused_variables)]

use image::{DynamicImage, GenericImage, ImageFormat, Pixel, Rgba};

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        spheres: vec![Sphere {
            center: Point3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            color: Color {
                red: 200,
                green: 20,
                blue: 50,
            },
        }],
    };

    let image = scene.render();

    image
        .save_with_format("image.png", ImageFormat::Png)
        .unwrap();
}

#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

impl std::ops::Add for Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn zero() -> Point3 {
        Point3::new(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, other: &Point3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn to_vector(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f32 {
        self.dot(&self).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let length = self.length();
        Vector3::new(self.x / length, self.y / length, self.z / length)
    }

    fn to_point(&self) -> Point3 {
        Point3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

#[derive(Debug, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(self.red, self.green, self.blue, 255)
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    color: Color,
}

impl Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let line = (self.center - ray.origin).to_vector();
        let adj = line.dot(&ray.direction);
        let d2 = line.dot(&line) - adj.powi(2);
        d2 < self.radius.powi(2)
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

#[derive(Debug, Clone)]
pub struct Scene {
    spheres: Vec<Sphere>,
    width: u32,
    height: u32,
    fov: f32,
}

impl Scene {
    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Color {
            red: 0,
            green: 0,
            blue: 0,
        };
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = self.spawn_prime_ray(x as f32, y as f32);
                let sphere = self.spheres.get(0).unwrap();
                if sphere.intersect(&ray) {
                    image.put_pixel(x, y, sphere.color.to_rgba());
                } else {
                    image.put_pixel(x, y, black.to_rgba());
                }
            }
        }
        image
    }
    pub fn spawn_prime_ray(&self, x: f32, y: f32) -> Ray {
        // We are assuming that our image is wider than it is tall.
        //  otherwise, the aspect ratio adjustment would be wrong.
        assert!(self.width > self.height);

        // + 0.5 to center the ray in the pixel
        let mut sensor_x = (x + 0.5) / self.width as f32;
        let mut sensor_y = (y + 0.5) / self.height as f32;

        // Convert coordinates to -1.0 to 1.0
        //  and flip y axis
        sensor_x = 2.0 * sensor_x - 1.0;
        sensor_y = 1.0 - (2.0 * sensor_y);

        let aspect_ratio = self.width as f32 / self.height as f32;
        sensor_x *= aspect_ratio;

        let fov_adj = (self.fov.to_radians() / 2.0).tan();
        sensor_x *= fov_adj;
        sensor_y *= fov_adj;

        Ray {
            origin: Point3::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }
}

#[test]
fn test_can_render_scene() {
    use image::GenericImageView;

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        spheres: vec![Sphere {
            center: Point3::zero(),
            radius: 1.0,
            color: Color {
                red: 200,
                green: 20,
                blue: 50,
            },
        }],
    };

    let img: DynamicImage = scene.render();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

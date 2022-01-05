#![allow(dead_code)]
#![allow(unused_variables)]

mod color;
mod vector;

use color::Color;
use image::{DynamicImage, GenericImage, ImageFormat};
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
        spheres: vec![Sphere {
            center: Vector3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            color: Color::new(255, 255, 255),
            albedo: 0.18,
        }],
    };

    let image = scene.render();

    image
        .save_with_format("image.png", ImageFormat::Png)
        .unwrap();
}

#[derive(Debug, Clone)]
pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub color: Color,
    pub albedo: f32,
}

impl Sphere {
    fn intersect_distance(&self, ray: &Ray) -> Option<f32> {
        // Trigonometry yay!
        let line = self.center - ray.origin;
        let adj = line.dot(&ray.direction);
        let distance2 = line.dot(&line) - adj.powi(2);
        let radius2 = self.radius.powi(2);

        // Check if the sphere and the ray intersect
        if distance2 > radius2 {
            return None;
        }

        // Calculate the thickness of the intersection, from surface of the sphere to the closest
        // point on the ray to the center of the sphere
        let thickness = (radius2 - distance2).sqrt();

        // Both intersection points
        let intersection_in = adj - thickness;
        let intersection_out = adj + thickness;

        // Is this necessary?
        // if intersection_in < 0.0 && intersection_out < 0.0 {
        //     return None;
        // }

        let distance = intersection_in.min(intersection_out);
        Some(distance)
    }

    pub fn surface_normal(&self, hit_point: Vector3) -> Vector3 {
        (hit_point - self.center).normalize()
    }

    pub fn calc_color(&self, ray: &Ray, light: &Light) -> Color {
        if let Some(distance) = self.intersect_distance(ray) {
            let hit_point = ray.origin + (ray.direction * distance);
            let surface_normal = self.surface_normal(hit_point);
            let light_direction = light.direction.normalize() * -1.0;
            // Amount of light that lands on the point
            let light_intensity = surface_normal.dot(&light_direction).max(0.0) * light.intensity;
            // Amount of light reflected
            let light_reflected = self.albedo / std::f32::consts::PI;

            // Combine all: color of the point, color of the light, light intensity, and light reflected
            let res_color = Vector3::new(
                (self.color.red as f32 / 255.0) * (light.color.red as f32 / 255.0),
                (self.color.green as f32 / 255.0) * (light.color.green as f32 / 255.0),
                (self.color.blue as f32 / 255.0) * (light.color.blue as f32 / 255.0),
            );
            let res_color = res_color * light_intensity * light_reflected * 255.0;
            Color::new(res_color.x as u8, res_color.y as u8, res_color.z as u8)
        } else {
            Color::new(100, 100, 100)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub light: Light,
}

impl Scene {
    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = self.spawn_prime_ray(x as f32, y as f32);
                let sphere = self.spheres.get(0).unwrap();
                let color = sphere.calc_color(&ray, &self.light);
                image.put_pixel(x, y, color.to_rgba());
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

        // Adjust for the aspect ratio
        let aspect_ratio = self.width as f32 / self.height as f32;
        sensor_x *= aspect_ratio;

        // Adjust for the fov
        let fov_adj = (self.fov.to_radians() / 2.0).tan();
        sensor_x *= fov_adj;
        sensor_y *= fov_adj;

        Ray {
            origin: Vector3::zero(),
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
        light: Light {
            direction: Vector3::new(0.0, 0.0, -1.0),
            color: Color::new(255, 255, 255),
            intensity: 1.0,
        },
        spheres: vec![Sphere {
            center: Vector3::zero(),
            radius: 1.0,
            color: Color::new(200, 20, 50),
            albedo: 1.0,
        }],
    };

    let img: DynamicImage = scene.render();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

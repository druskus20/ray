use image::{DynamicImage, GenericImage};

use crate::{color::Color, object::Object, vector::Vector3};

#[derive(Debug, Clone)]
pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub objects: Vec<Object>,
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
                let sphere = self.objects.get(0).unwrap();
                let color = sphere.calc_color(&ray, &self.light);
                image.put_pixel(x, y, color.to_rgba());
            }
        }
        image
    }

    pub fn spawn_prime_ray(&self, x: f32, y: f32) -> Ray {
        // + 0.5 to center the ray in the pixel
        let mut sensor_x = (x + 0.5) / self.width as f32;
        let mut sensor_y = (y + 0.5) / self.height as f32;

        // Convert coordinates to -1.0 to 1.0
        //  and flip y axis
        sensor_x = 2.0 * sensor_x - 1.0;
        sensor_y = 1.0 - (2.0 * sensor_y);

        // Adjust for the aspect ratio
        if self.width > self.height {
            let aspect_ratio = self.width as f32 / self.height as f32;
            sensor_x *= aspect_ratio;
        } else {
            let aspect_ratio = self.height as f32 / self.width as f32;
            sensor_y *= aspect_ratio;
        }

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

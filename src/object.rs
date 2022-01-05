use crate::{
    color::Color,
    render::{Light, Ray},
    Vector3,
};

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn color(&self) -> Color {
        match self {
            Object::Sphere(s) => s.color,
            Object::Plane(p) => p.color,
        }
    }

    pub fn albedo(&self) -> f32 {
        match self {
            Object::Sphere(s) => s.albedo,
            Object::Plane(p) => p.albedo,
        }
    }

    pub fn surface_normal(&self, hit_point: Vector3) -> Vector3 {
        match self {
            Object::Sphere(x) => (hit_point - x.center).normalize(),
            Object::Plane(x) => -x.normal, // TODO: Why -?
        }
    }

    // TODO: This shouldnt belong to Object probably
    pub fn calc_color(&self, ray: &Ray, light: &Light) -> Option<Color> {
        if let Some(distance) = self.intersect_distance(ray) {
            let hit_point = ray.origin + (ray.direction * distance);
            let surface_normal = self.surface_normal(hit_point);
            let light_direction = -light.direction.normalize();

            // Amount of light that lands on the point
            let light_intensity = surface_normal.dot(&light_direction).max(0.0) * light.intensity;
            // Amount of light reflected
            let light_reflected = self.albedo() / std::f32::consts::PI;

            // Combine all: color of the point, color of the light, light intensity, and light reflected
            let res_color = Vector3::new(
                (self.color().red as f32 / 255.0) * (light.color.red as f32 / 255.0),
                (self.color().green as f32 / 255.0) * (light.color.green as f32 / 255.0),
                (self.color().blue as f32 / 255.0) * (light.color.blue as f32 / 255.0),
            );

            let res_color = res_color * light_intensity * light_reflected * 255.0;
            Some(Color::new(
                res_color.x as u8,
                res_color.y as u8,
                res_color.z as u8,
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
    pub origin: Vector3,
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub color: Color,
    pub albedo: f32,
}

pub trait Intersectable {
    fn intersect_distance(&self, ray: &Ray) -> Option<f32>;
}

impl Intersectable for Object {
    fn intersect_distance(&self, ray: &Ray) -> Option<f32> {
        match self {
            Object::Sphere(s) => s.intersect_distance(ray),
            Object::Plane(p) => p.intersect_distance(ray),
        }
    }
}

impl Intersectable for Plane {
    fn intersect_distance(&self, ray: &Ray) -> Option<f32> {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection

        let normal = self.normal;
        let denom = normal.dot(&ray.direction);

        // If the denominator is close to 0, the ray is parallel to the plane (eq tends to infinity)
        if denom.abs() < 1e-6 {
            return None;
        }

        let num = (self.origin - ray.origin).dot(&normal);
        let distance = num / denom;

        Some(distance)
    }
}

impl Intersectable for Sphere {
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

        // TODO: Is this necessary?
        // if intersection_in < 0.0 && intersection_out < 0.0 {
        //     return None;
        // }

        let distance = intersection_in.min(intersection_out);
        Some(distance)
    }
}

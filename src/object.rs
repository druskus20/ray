use crate::{color::Color, render::Ray, Vector3};

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

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f32,
    pub object: Object,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Object::Sphere(s) => s.intersect(ray),
            Object::Plane(p) => p.intersect(ray),
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection

        let normal = self.normal;
        let denom = normal.dot(&ray.direction);

        // If the denominator is close to 0, the ray is parallel to the plane (eq tends to infinity)
        if denom.abs() < 1e-6 {
            return None;
        }

        let num = (self.origin - ray.origin).dot(&normal);
        let distance = num / denom;

        Some(Intersection {
            distance,
            object: Object::Plane(self.clone()), // TODO: This is kinda weird
        })
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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
        if intersection_in < 0.0 && intersection_out < 0.0 {
            return None;
        }

        let distance = intersection_in.min(intersection_out);
        Some(Intersection {
            distance,
            object: Object::Sphere(self.clone()), // TODO: This is kinda weird
        })
    }
}

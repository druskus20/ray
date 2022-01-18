use crate::{color::Color, render::Ray, vector::Vector2, Vector3};

#[derive(Debug, Clone)]
pub struct Object {
    pub material: Material,
    pub mesh: Mesh,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub albedo: f32,
}

impl Material {
    pub fn new(color: Color, albedo: f32) -> Self {
        Self { color, albedo }
    }
}

impl Object {
    pub fn new(material: Material, mesh: Mesh) -> Self {
        Self { material, mesh }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(distance) = self.mesh.intersect_distance(ray) {
            Some(Intersection {
                distance,
                object: self.clone(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum Mesh {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub normal: Vector3,
    pub origin: Vector3,
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f32,
    pub object: Object,
}

pub trait Intersectable {
    fn intersect_distance(&self, ray: &Ray) -> Option<f32>;
    fn surface_normal(&self, hit_point: Vector3) -> Vector3;
    fn texture_coords(&self, hit_point: Vector3) -> Vector2;
}

impl Intersectable for Mesh {
    fn intersect_distance(&self, ray: &Ray) -> Option<f32> {
        match &self {
            Mesh::Sphere(s) => s.intersect_distance(ray),
            Mesh::Plane(p) => p.intersect_distance(ray),
        }
    }

    fn surface_normal(&self, hit_point: Vector3) -> Vector3 {
        match &self {
            Mesh::Sphere(s) => s.surface_normal(hit_point),
            Mesh::Plane(p) => p.surface_normal(hit_point),
        }
    }

    fn texture_coords(&self, hit_point: Vector3) -> Vector2 {
        match &self {
            Mesh::Sphere(s) => s.texture_coords(hit_point),
            Mesh::Plane(p) => p.texture_coords(hit_point),
        }
    }
}

impl Intersectable for Plane {
    fn intersect_distance(&self, ray: &Ray) -> Option<f32> {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection

        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);

        // If the denominator is close to 0, the ray is parallel to the plane (eq tends to infinity)
        if denom < 1e-6 {
            return None;
        }

        let num = (self.origin - ray.origin).dot(&normal);
        let distance = num / denom;

        Some(distance)
    }

    fn surface_normal(&self, _hit_point: Vector3) -> Vector3 {
        -self.normal
    }

    fn texture_coords(&self, hit_point: Vector3) -> Vector2 {
        todo!()
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
        if intersection_in < 0.0 && intersection_out < 0.0 {
            return None;
        }

        let distance = intersection_in.min(intersection_out);
        Some(distance)
    }

    fn surface_normal(&self, hit_point: Vector3) -> Vector3 {
        (hit_point - self.center).normalize()
    }

    fn texture_coords(&self, hit_point: Vector3) -> Vector2 {
        todo!()
    }
}

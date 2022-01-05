use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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

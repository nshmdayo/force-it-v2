use cgmath::Vector3;

/// 3D vector utility struct for force calculations and particle physics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Create a new vector with zero values
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Create a new vector with specified values
    pub fn from_coords(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Add another vector to this one
    pub fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    /// Subtract current vector from target vector and store result
    pub fn sub(&mut self, target: &Vec3, current: &Vec3) {
        self.x = target.x - current.x;
        self.y = target.y - current.y;
        self.z = target.z - current.z;
    }

    /// Multiply vector by scalar
    pub fn mult(&mut self, k: f64) {
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    /// Calculate dot product with another vector
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate magnitude squared
    pub fn mag_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Calculate magnitude (distance from origin)
    pub fn magnitude(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    /// Normalize the vector to unit length
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag != 0.0 {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }

    /// Copy values from another vector
    pub fn copy(&mut self, other: &Vec3) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }

    /// Reset vector to zero
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    /// Convert to cgmath Vector3 for graphics operations
    pub fn to_vector3(&self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Create from cgmath Vector3
    pub fn from_vector3(v: Vector3<f32>) -> Self {
        Self {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::from_coords(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::from_coords(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::from_coords(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }
}

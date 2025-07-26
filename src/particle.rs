use crate::vector::Vec3;
use std::time::{SystemTime, UNIX_EPOCH};

/// Particle struct representing a physics-enabled sphere
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub radius: f64,
    pub created_time: u64,
    pub delete_flag: bool,
    diff: Vec3,
    original_position: Vec3,
}

impl Particle {
    /// Create a new particle at specified position
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let position = Vec3::from_coords(x, y, z);
        Self {
            position,
            velocity: Vec3::new(),
            radius: 1.0,
            created_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            delete_flag: false,
            diff: Vec3::new(),
            original_position: position,
        }
    }

    /// Set the radius of the particle
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Move the particle by its velocity
    pub fn move_particle(&mut self) {
        self.position.add(&self.velocity);
        self.diff.reset();
    }

    /// Apply gravity force towards original position
    pub fn gravity(&mut self) {
        if self.position.z < 0.0 {
            // Reset to original position if below ground
            self.position.copy(&self.original_position);
            self.velocity.reset();
        } else {
            // Apply spring force towards original position
            let mut force = Vec3::new();
            force.sub(&self.original_position, &self.position);
            force.mult(0.0005);
            self.velocity.add(&force);
        }
    }

    /// Set around module for inter-particle forces
    pub fn set_around_module(&mut self, v: &Vec3) {
        self.diff.add(v);
    }

    /// Apply module gravity based on surrounding particles
    pub fn module_gravity(&mut self, d: i32) {
        let a = 1.0 / (4.0 * d as f64 * (d as f64 + 1.0));
        self.diff.mult(a);
        self.velocity.add(&self.diff);
    }

    /// Add velocity to the particle
    pub fn add_velocity(&mut self, v: &Vec3) {
        self.velocity.add(v);
    }

    /// Get current position
    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }

    /// Get current velocity
    pub fn get_velocity(&self) -> &Vec3 {
        &self.velocity
    }

    /// Check if particle has exceeded lifetime
    pub fn is_expired(&self, max_lifetime_ms: u64) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        current_time - self.created_time > max_lifetime_ms
    }

    /// Check collision with another particle
    pub fn collides_with(&self, other: &Particle) -> bool {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        let dz = self.position.z - other.position.z;
        let distance_squared = dx * dx + dy * dy + dz * dz;
        let radius_sum = self.radius + other.radius;
        distance_squared < radius_sum * radius_sum
    }

    /// Apply collision response with another particle
    pub fn apply_collision(&mut self, other: &Particle, m1: f64, m2: f64) {
        let a = 1.0 / (m1 + m2);
        
        let mut v1 = Vec3::new();
        v1.copy(&self.velocity);
        v1.mult(m1 - m2);
        
        let mut v2 = Vec3::new();
        v2.copy(&other.velocity);
        v2.mult(2.0 * m2);
        
        v1.add(&v2);
        v1.mult(a);
        
        self.velocity.add(&v1);
    }
}

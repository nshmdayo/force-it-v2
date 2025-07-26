use crate::particle::Particle;
use crate::vector::Vec3;

pub struct PhysicsSystem {
    pub wall_particles: Vec<Particle>,
    pub force_particles: Vec<Particle>,
    pub sphere_width_number: i32,
    pub sphere_height_number: i32,
    pub d: i32,
    pub m1: f64, // mass of wall particles
    pub m2: f64, // mass of force particles
    pub ball_radius: f64,
    pub force_radius: f64,
    pub force_power_rate: f64,
    pub make_force_num: usize,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        let sphere_width_number = 21;
        let sphere_height_number = 21;
        let d = 10;
        let ball_radius = 5.0;
        let force_radius = 1.0;

        let mut wall_particles = Vec::new();

        // Create wall of particles
        for y in -20..0 {
            for x in -20..20 {
                let particle = Particle::new(
                    (x as f64) * ball_radius * 2.0,
                    (y as f64) * ball_radius * 2.0,
                    0.0,
                );
                let mut p = particle;
                p.set_radius(ball_radius);
                wall_particles.push(p);
            }
        }

        Self {
            wall_particles,
            force_particles: Vec::new(),
            sphere_width_number,
            sphere_height_number,
            d,
            m1: 1.0,
            m2: 1.0,
            ball_radius,
            force_radius,
            force_power_rate: 0.4,
            make_force_num: 500,
        }
    }

    pub fn update(&mut self) {
        self.update_wall_module_forces();
        self.update_wall_physics();
        self.update_force_physics();
        self.handle_collisions();
        self.remove_expired_forces();
    }

    fn update_wall_module_forces(&mut self) {
        let wall_count = self.wall_particles.len();
        let width = 40; // -20 to 20
        
        // Calculate module forces for wall particles
        for y in self.d..(20 - self.d) {
            for x in self.d..(40 - self.d) {
                let center_idx = (y + 20) * width + (x - 20 + 20);
                if center_idx < 0 || center_idx >= wall_count as i32 {
                    continue;
                }

                let mut module_force = Vec3::new();
                
                for j in -self.d..=self.d {
                    for i in -self.d..=self.d {
                        let neighbor_idx = ((y + j + 20) * width + (x + i - 20 + 20)) as usize;
                        if neighbor_idx < wall_count {
                            let neighbor_pos = self.wall_particles[neighbor_idx].position;
                            module_force.add(&neighbor_pos);
                        }
                    }
                }

                if (center_idx as usize) < wall_count {
                    self.wall_particles[center_idx as usize].set_around_module(&module_force);
                }
            }
        }
    }

    fn update_wall_physics(&mut self) {
        for particle in &mut self.wall_particles {
            particle.gravity();
            particle.module_gravity(self.d);
            particle.move_particle();
        }
    }

    fn update_force_physics(&mut self) {
        for particle in &mut self.force_particles {
            particle.move_particle();
        }
    }

    fn handle_collisions(&mut self) {
        let mut forces_to_remove = Vec::new();

        for (force_idx, force_particle) in self.force_particles.iter().enumerate() {
            for wall_particle in &mut self.wall_particles {
                if force_particle.collides_with(wall_particle) {
                    wall_particle.apply_collision(force_particle, self.m1, self.m2);
                    forces_to_remove.push(force_idx);
                    break;
                }
            }
        }

        // Remove collided forces (in reverse order to maintain indices)
        for &idx in forces_to_remove.iter().rev() {
            if idx < self.force_particles.len() {
                self.force_particles.remove(idx);
            }
        }
    }

    fn remove_expired_forces(&mut self) {
        self.force_particles.retain(|particle| !particle.is_expired(5000)); // 5 second lifetime
    }

    pub fn create_force_particles(&mut self, hand_position: &Vec3, hand_velocity: &Vec3, spread_distance: f64) {
        if hand_velocity.magnitude() > 0.001 {
            for _ in 0..self.make_force_num {
                let spread_x = spread_distance * (rand::random::<f64>() - 0.5) * 0.05;
                let spread_y = spread_distance * (rand::random::<f64>() - 0.5) * 0.05;
                let spread_z = spread_distance * (rand::random::<f64>() - 0.5) * 0.05;

                let mut force_particle = Particle::new(
                    hand_position.x + spread_x,
                    hand_position.y + spread_y,
                    hand_position.z + spread_z,
                );
                force_particle.set_radius(self.force_radius);

                let mut velocity = *hand_velocity;
                velocity.mult(self.force_power_rate);
                force_particle.add_velocity(&velocity);

                self.force_particles.push(force_particle);
            }
        }
    }

    pub fn get_all_particles(&self) -> Vec<&Particle> {
        let mut all_particles = Vec::new();
        all_particles.extend(self.wall_particles.iter());
        all_particles.extend(self.force_particles.iter());
        all_particles
    }
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}

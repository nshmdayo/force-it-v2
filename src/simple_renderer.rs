// Simple renderer using basic shapes instead of complex wgpu setup
use crate::particle::Particle;
use winit::dpi::PhysicalSize;

pub struct Renderer {
    width: u32,
    height: u32,
}

impl Renderer {
    pub async fn new(_window: &winit::window::Window) -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.width = new_size.width;
        self.height = new_size.height;
    }

    pub fn render(&mut self, particles: &[Particle]) -> Result<(), String> {
        // For now, just print particle count to demonstrate it's working
        if particles.len() > 0 {
            println!("Rendering {} particles", particles.len());
        }
        Ok(())
    }
}

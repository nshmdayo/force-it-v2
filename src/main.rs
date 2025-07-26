mod vector;
mod particle;
mod physics;
mod simple_renderer;
mod input;

use winit::{
    event::*,
    event_loop::{EventLoop},
    window::{Window, WindowId},
    dpi::PhysicalSize,
};

use physics::PhysicsSystem;
use simple_renderer::Renderer;
use input::InputSystem;

const SCENE_WIDTH: u32 = 1920;
const SCENE_HEIGHT: u32 = 1080;

struct App {
    window: Option<Window>,
    physics_system: Option<PhysicsSystem>,
    input_system: Option<InputSystem>,
    renderer: Option<Renderer>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            physics_system: None,
            input_system: None,
            renderer: None,
        }
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("ForceIt - Rust Implementation")
            .with_inner_size(PhysicalSize::new(SCENE_WIDTH, SCENE_HEIGHT));
        
        let window = event_loop.create_window(window_attributes).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &self.window {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    if let Some(renderer) = &mut self.renderer {
                        renderer.resize(physical_size);
                    }
                    if let Some(input_system) = &mut self.input_system {
                        input_system.resize(physical_size.width, physical_size.height);
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if let Some(input_system) = &mut self.input_system {
                        input_system.update_mouse_position(position.x, position.y);
                    }
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    if let Some(input_system) = &mut self.input_system {
                        input_system.update_mouse_button(button, state);
                    }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let winit::keyboard::PhysicalKey::Code(keycode) = event.physical_key {
                        match keycode {
                            winit::keyboard::KeyCode::Space if event.state == ElementState::Pressed => {
                                if let Some(input_system) = &mut self.input_system {
                                    if !input_system.is_calibrated {
                                        input_system.calibrate();
                                        println!("System calibrated! Use mouse to interact with particles.");
                                    }
                                }
                            }
                            winit::keyboard::KeyCode::Escape => {
                                event_loop.exit();
                            }
                            _ => {}
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Initialize systems if needed
                    if self.physics_system.is_none() {
                        self.physics_system = Some(PhysicsSystem::new());
                        self.input_system = Some(InputSystem::new(SCENE_WIDTH, SCENE_HEIGHT));
                        
                        // Initialize renderer asynchronously
                        if self.renderer.is_none() {
                            let renderer = pollster::block_on(Renderer::new(window));
                            self.renderer = Some(renderer);
                        }
                    }

                    // Update
                    if let Some(physics_system) = &mut self.physics_system {
                        physics_system.update();

                        if let Some(input_system) = &self.input_system {
                            if input_system.is_calibrated {
                                let right_hand_vector = input_system.get_right_hand_vector();
                                if right_hand_vector.magnitude() > 0.001 && input_system.body_joints.right_hand.is_open {
                                    let hand_position = input_system.body_joints.right_hand.position;
                                    let spread_distance = input_system.get_spread_distance();
                                    
                                    physics_system.create_force_particles(
                                        &hand_position,
                                        &right_hand_vector,
                                        spread_distance,
                                    );
                                }
                            }
                        }
                    }

                    // Render
                    if let (Some(renderer), Some(physics_system)) = (&mut self.renderer, &self.physics_system) {
                        let particles = physics_system.get_all_particles();
                        let particle_refs: Vec<_> = particles.into_iter().cloned().collect();
                        match renderer.render(&particle_refs) {
                            Ok(_) => {}
                            Err(e) => eprintln!("Render error: {:?}", e),
                        }
                    }

                    window.request_redraw();
                }
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    
    println!("ForceIt - Body-based Interaction System");
    println!("Controls:");
    println!("  Space - Calibrate system");
    println!("  Mouse - Control right hand");
    println!("  Left Click - Open hand to create forces");
    println!("  Escape - Exit");

    event_loop.run_app(&mut app).unwrap();
}

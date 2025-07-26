use crate::vector::Vec3;
use winit::event::{MouseButton, ElementState};

#[derive(Debug, Clone)]
pub struct HandState {
    pub position: Vec3,
    pub velocity: Vec3,
    pub is_open: bool,
    pub is_tracked: bool,
}

impl Default for HandState {
    fn default() -> Self {
        Self {
            position: Vec3::new(),
            velocity: Vec3::new(),
            is_open: false,
            is_tracked: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BodyJoints {
    pub right_hand: HandState,
    pub left_hand: HandState,
    pub spine_shoulder: Vec3,
    pub spine_base: Vec3,
    pub shoulder_right: Vec3,
}

impl Default for BodyJoints {
    fn default() -> Self {
        Self {
            right_hand: HandState::default(),
            left_hand: HandState::default(),
            spine_shoulder: Vec3::new(),
            spine_base: Vec3::new(),
            shoulder_right: Vec3::new(),
        }
    }
}

pub struct InputSystem {
    pub body_joints: BodyJoints,
    pub is_calibrated: bool,
    mouse_position: (f64, f64),
    previous_mouse_position: (f64, f64),
    mouse_pressed: bool,
    window_size: (u32, u32),
}

impl InputSystem {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self {
            body_joints: BodyJoints::default(),
            is_calibrated: false,
            mouse_position: (0.0, 0.0),
            previous_mouse_position: (0.0, 0.0),
            mouse_pressed: false,
            window_size: (window_width, window_height),
        }
    }

    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        self.previous_mouse_position = self.mouse_position;
        self.mouse_position = (x, y);
        
        // Convert mouse coordinates to 3D space
        let normalized_x = (x / self.window_size.0 as f64 - 0.5) * 200.0;
        let normalized_y = (y / self.window_size.1 as f64 - 0.5) * 200.0;
        let z = if self.mouse_pressed { -50.0 } else { -100.0 };

        // Calculate velocity from mouse movement
        let velocity_x = normalized_x - (self.previous_mouse_position.0 / self.window_size.0 as f64 - 0.5) * 200.0;
        let velocity_y = normalized_y - (self.previous_mouse_position.1 / self.window_size.1 as f64 - 0.5) * 200.0;

        self.body_joints.right_hand.position = Vec3::from_coords(normalized_x, normalized_y, z);
        self.body_joints.right_hand.velocity = Vec3::from_coords(velocity_x * 0.1, velocity_y * 0.1, 0.0);
        self.body_joints.right_hand.is_tracked = true;
    }

    pub fn update_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left {
            match state {
                ElementState::Pressed => {
                    self.mouse_pressed = true;
                    self.body_joints.right_hand.is_open = true;
                }
                ElementState::Released => {
                    self.mouse_pressed = false;
                    self.body_joints.right_hand.is_open = false;
                }
            }
        }
    }

    pub fn calibrate(&mut self) {
        // Simulate calibration by setting default body positions
        self.body_joints.spine_base = Vec3::from_coords(0.0, 0.0, 0.0);
        self.body_joints.spine_shoulder = Vec3::from_coords(0.0, 50.0, 0.0);
        self.body_joints.shoulder_right = Vec3::from_coords(30.0, 40.0, 0.0);
        self.body_joints.left_hand.position = Vec3::from_coords(-50.0, 0.0, -100.0);
        
        self.is_calibrated = true;
        println!("Input system calibrated - use mouse to control right hand");
    }

    pub fn get_right_hand_vector(&self) -> Vec3 {
        if self.body_joints.right_hand.is_open && self.body_joints.right_hand.is_tracked {
            self.body_joints.right_hand.position - self.body_joints.shoulder_right
        } else {
            Vec3::new()
        }
    }

    pub fn get_spread_distance(&self) -> f64 {
        let left_hand = &self.body_joints.left_hand.position;
        let spine_base = &self.body_joints.spine_base;
        
        let dx = spine_base.x - left_hand.x;
        let dy = spine_base.y - left_hand.y;
        let dz = spine_base.z - left_hand.z;
        
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.window_size = (width, height);
    }
}

use crate::vector::Vec3;
use crate::pose_detection::PoseDetector;
use winit::event::{MouseButton, ElementState};

#[derive(Debug, Clone)]
pub struct HandState {
    pub position: Vec3,
    pub velocity: Vec3,
    pub is_open: bool,
    pub is_tracked: bool,
    pub confidence: f32,
    pub previous_position: Vec3,
}

impl Default for HandState {
    fn default() -> Self {
        Self {
            position: Vec3::new(),
            velocity: Vec3::new(),
            is_open: false,
            is_tracked: false,
            confidence: 0.0,
            previous_position: Vec3::new(),
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
    pub shoulder_left: Vec3,
    pub neck: Vec3,
}

impl Default for BodyJoints {
    fn default() -> Self {
        Self {
            right_hand: HandState::default(),
            left_hand: HandState::default(),
            spine_shoulder: Vec3::new(),
            spine_base: Vec3::new(),
            shoulder_right: Vec3::new(),
            shoulder_left: Vec3::new(),
            neck: Vec3::new(),
        }
    }
}

pub struct InputSystem {
    pub body_joints: BodyJoints,
    pub is_calibrated: bool,
    pose_detector: Option<PoseDetector>,
    window_size: (u32, u32),
    last_update_time: u64,
    // Fallback mouse input for debugging
    mouse_position: (f64, f64),
    mouse_pressed: bool,
    use_pose_detection: bool,
}

impl InputSystem {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let mut system = Self {
            body_joints: BodyJoints::default(),
            is_calibrated: false,
            pose_detector: None,
            window_size: (window_width, window_height),
            last_update_time: 0,
            mouse_position: (0.0, 0.0),
            mouse_pressed: false,
            use_pose_detection: true,
        };

        // Try to initialize pose detection
        match PoseDetector::new() {
            Ok(mut detector) => {
                match detector.initialize_openpose() {
                    Ok(_) => {
                        println!("✅ Pose detection initialized successfully");
                        system.pose_detector = Some(detector);
                    }
                    Err(e) => {
                        println!("⚠️  OpenPose initialization failed: {}", e);
                        println!("🔄 Falling back to mouse input mode");
                        system.use_pose_detection = false;
                    }
                }
            }
            Err(e) => {
                println!("⚠️  Camera initialization failed: {}", e);
                println!("🔄 Falling back to mouse input mode");
                system.use_pose_detection = false;
            }
        }

        system
    }

    pub fn update(&mut self) {
        if self.use_pose_detection {
            self.update_pose_detection();
        }
    }

    fn update_pose_detection(&mut self) {
        if let Some(ref mut detector) = self.pose_detector {
            if let Err(e) = detector.update() {
                println!("Pose detection update error: {}", e);
                return;
            }

            if detector.is_ready() {
                // Get positions from detector without borrowing self
                let right_hand_pos = detector.get_hand_position("right");
                let left_hand_pos = detector.get_hand_position("left");
                let right_hand_open = detector.is_hand_open("right");
                let left_hand_open = detector.is_hand_open("left");
                let neck_pos = detector.get_body_keypoint("Neck");
                let right_shoulder_pos = detector.get_body_keypoint("RShoulder");
                let left_shoulder_pos = detector.get_body_keypoint("LShoulder");

                // Update body joints with the retrieved data
                self.update_body_joints_from_data(
                    right_hand_pos, left_hand_pos,
                    right_hand_open, left_hand_open,
                    neck_pos, right_shoulder_pos, left_shoulder_pos
                );
            }
        }
    }

    fn update_body_joints_from_data(
        &mut self,
        right_hand_pos: Option<Vec3>,
        left_hand_pos: Option<Vec3>,
        right_hand_open: bool,
        left_hand_open: bool,
        neck_pos: Option<Vec3>,
        right_shoulder_pos: Option<Vec3>,
        left_shoulder_pos: Option<Vec3>,
    ) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Update right hand
        if let Some(right_hand_position) = right_hand_pos {
            self.body_joints.right_hand.previous_position = self.body_joints.right_hand.position;
            self.body_joints.right_hand.position = right_hand_position;
            self.body_joints.right_hand.is_tracked = true;
            self.body_joints.right_hand.is_open = right_hand_open;
            
            // Calculate velocity
            if self.last_update_time > 0 {
                let dt = (current_time - self.last_update_time) as f64 / 1000.0; // seconds
                if dt > 0.0 {
                    let velocity = (self.body_joints.right_hand.position - self.body_joints.right_hand.previous_position) * (1.0 / dt);
                    self.body_joints.right_hand.velocity = velocity;
                }
            }
        } else {
            self.body_joints.right_hand.is_tracked = false;
        }

        // Update left hand
        if let Some(left_hand_position) = left_hand_pos {
            self.body_joints.left_hand.previous_position = self.body_joints.left_hand.position;
            self.body_joints.left_hand.position = left_hand_position;
            self.body_joints.left_hand.is_tracked = true;
            self.body_joints.left_hand.is_open = left_hand_open;
            
            // Calculate velocity
            if self.last_update_time > 0 {
                let dt = (current_time - self.last_update_time) as f64 / 1000.0;
                if dt > 0.0 {
                    let velocity = (self.body_joints.left_hand.position - self.body_joints.left_hand.previous_position) * (1.0 / dt);
                    self.body_joints.left_hand.velocity = velocity;
                }
            }
        } else {
            self.body_joints.left_hand.is_tracked = false;
        }

        // Update body keypoints
        if let Some(neck_position) = neck_pos {
            self.body_joints.neck = neck_position;
        }

        if let Some(right_shoulder_position) = right_shoulder_pos {
            self.body_joints.shoulder_right = right_shoulder_position;
        }

        if let Some(left_shoulder_position) = left_shoulder_pos {
            self.body_joints.shoulder_left = left_shoulder_position;
        }

        // Estimate spine positions based on shoulders and neck
        if self.body_joints.neck.magnitude() > 0.0 {
            self.body_joints.spine_shoulder = self.body_joints.neck + Vec3::from_coords(0.0, 30.0, 0.0);
            self.body_joints.spine_base = self.body_joints.spine_shoulder + Vec3::from_coords(0.0, 150.0, 0.0);
        }

        self.last_update_time = current_time;
    }

    // Fallback mouse input methods (for debugging when camera is not available)
    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        if !self.use_pose_detection {
            let prev_pos = self.body_joints.right_hand.position;
            
            let normalized_x = (x / self.window_size.0 as f64 - 0.5) * 200.0;
            let normalized_y = (y / self.window_size.1 as f64 - 0.5) * 200.0;
            let z = if self.mouse_pressed { -50.0 } else { -100.0 };

            self.body_joints.right_hand.position = Vec3::from_coords(normalized_x, normalized_y, z);
            self.body_joints.right_hand.is_tracked = true;
            
            // Calculate velocity for mouse input
            let velocity = self.body_joints.right_hand.position - prev_pos;
            self.body_joints.right_hand.velocity = velocity * 10.0; // Scale for better effect
        }
    }

    pub fn update_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        if !self.use_pose_detection && button == MouseButton::Left {
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
        if self.use_pose_detection {
            if let Some(ref detector) = self.pose_detector {
                if detector.is_ready() {
                    println!("🎯 Pose detection calibrated!");
                    println!("📸 Camera is tracking your movements");
                    println!("✋ Move your hands to interact with particles");
                    self.is_calibrated = true;
                } else {
                    println!("⚠️  Please ensure you are visible to the camera");
                }
            }
        } else {
            // Fallback mouse calibration
            self.body_joints.spine_base = Vec3::from_coords(0.0, 0.0, 0.0);
            self.body_joints.spine_shoulder = Vec3::from_coords(0.0, 50.0, 0.0);
            self.body_joints.shoulder_right = Vec3::from_coords(30.0, 40.0, 0.0);
            self.body_joints.left_hand.position = Vec3::from_coords(-50.0, 0.0, -100.0);
            
            self.is_calibrated = true;
            println!("🖱️  Mouse input calibrated - use mouse to control right hand");
        }
    }

    pub fn get_right_hand_vector(&self) -> Vec3 {
        if self.body_joints.right_hand.is_open && self.body_joints.right_hand.is_tracked {
            // Return velocity-based vector for more dynamic interaction
            let mut hand_vector = self.body_joints.right_hand.velocity;
            
            // Normalize and scale the vector
            if hand_vector.magnitude() > 0.1 {
                hand_vector.mult(0.1); // Scale down for reasonable force
                return hand_vector;
            }
        }
        Vec3::new()
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

    pub fn get_detection_status(&self) -> String {
        if self.use_pose_detection {
            if let Some(ref detector) = self.pose_detector {
                if detector.is_ready() {
                    format!("📸 Camera: Active | 👤 Pose: Detected | ✋ Right Hand: {} | 🤚 Left Hand: {}", 
                        if self.body_joints.right_hand.is_tracked { "Tracked" } else { "Lost" },
                        if self.body_joints.left_hand.is_tracked { "Tracked" } else { "Lost" }
                    )
                } else {
                    "📸 Camera: Active | 👤 Pose: Not Detected".to_string()
                }
            } else {
                "📸 Camera: Not Available".to_string()
            }
        } else {
            "🖱️ Mouse Input Mode".to_string()
        }
    }
}

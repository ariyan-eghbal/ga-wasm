//! Heart animation using Rust, WebAssembly and WebGL
//! lib.rs

use crate::helpers::*;
use js_sys::Float32Array;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram};
use web_sys::{HtmlCanvasElement, Performance};

// Animation phases
#[derive(PartialEq, Copy, Clone)]
enum Phase {
    Initializing,
    Formed,
    Exploding,
    Reforming,
}

// Particle structure
struct Particle {
    x: f32,
    y: f32,
    target_x: f32,
    target_y: f32,
    vx: f32,
    vy: f32,
    is_outline: bool,
    size: f32,
    alpha: f32,
    reform_speed: f32,
    initial_speed: f32,
}

impl Particle {
    fn new(target_x: f32, target_y: f32, is_outline: bool, size: f32, rng: &mut SmallRng) -> Self {
        let angle = rng.gen::<f32>() * 2.0 * PI;
        let distance = rng.gen_range(400.0..600.0);

        Particle {
            x: angle.cos() * distance,
            y: angle.sin() * distance,
            target_x,
            target_y,
            vx: 0.0,
            vy: 0.0,
            is_outline,
            size,
            alpha: 255.0,
            reform_speed: rng.gen_range(0.05..0.08),
            initial_speed: rng.gen_range(0.02..0.04),
        }
    }

    fn explode(&mut self, rng: &mut SmallRng) {
        let angle = self.y.atan2(self.x);
        let force = if self.is_outline {
            rng.gen_range(2.0..4.0)
        } else {
            rng.gen_range(1.0..3.0)
        };
        self.vx = angle.cos() * force;
        self.vy = angle.sin() * force;
    }

    fn update(&mut self, phase: Phase) {
        match phase {
            Phase::Initializing => {
                let dx = self.target_x - self.x;
                let dy = self.target_y - self.y;
                self.x += dx * self.initial_speed;
                self.y += dy * self.initial_speed;
            }
            Phase::Exploding => {
                self.x += self.vx;
                self.y += self.vy;
                self.vy += 0.05;
                self.vx *= 0.95;
                self.vy *= 0.95;
                self.alpha = f32::max(self.alpha - 1.0, 100.0);
            }
            Phase::Reforming => {
                let dx = self.target_x - self.x;
                let dy = self.target_y - self.y;
                self.x += dx * self.reform_speed;
                self.y += dy * self.reform_speed;
                self.alpha = f32::min(self.alpha + 5.0, 255.0);
            }
            Phase::Formed => {}
        }
    }

    fn is_near_target(&self) -> bool {
        let dx = self.x - self.target_x;
        let dy = self.y - self.target_y;
        (dx * dx + dy * dy).sqrt() < 2.0
    }
}

struct HeartAnimation {
    particles: Vec<Particle>,
    phase: Phase,
    heart_rate: f32,
    last_beat: f64,
    gl: GL,
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    start_time: f64,
    rng: SmallRng,
    canvas_width: f32,
    canvas_height: f32,
}

impl HeartAnimation {
    fn new(gl: GL, width: f32, height: f32) -> Result<Self, JsValue> {
        let program = setup_shaders(&gl)?;
        let vertex_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;

        gl.use_program(Some(&program));
        match gl.get_uniform_location(&program, "color_multiplier") {
            Some(color_location) => {
                gl.uniform3f(Some(&color_location), 1.0, 0.0, 0.29);
            }
            None => {
                console_error!("color_multiplier uniform not found");
            }
        };

        let performance = window().unwrap().performance().unwrap();
        let start_time = performance.now();

        let rng = SmallRng::seed_from_u64(42);
        let mut heart = HeartAnimation {
            particles: Vec::new(),
            phase: Phase::Initializing,
            heart_rate: 60.0,
            last_beat: 0.0,
            gl,
            program,
            vertex_buffer,
            start_time,
            rng,
            canvas_width: width,
            canvas_height: height,
        };

        heart.initialize_particles();
        Ok(heart)
    }

    fn get_heart_x(t: f32) -> f32 {
        16.0 * (t.sin().powi(3))
    }

    fn get_heart_y(t: f32) -> f32 {
        13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos()
    }

    fn is_inside_heart(x: f32, y: f32) -> bool {
        let px = x / 8.0;
        let py = -y / 8.0;
        let px2 = px * px;
        let py2 = py * py;
        py2 < (4.0 - px2) * (1.0 - px2)
    }

    fn initialize_particles(&mut self) {
        // Create outline particles
        for i in 0..63 {
            let t = (i as f32) * 2.0 * PI / 63.0;
            let x = Self::get_heart_x(t) * 8.0;
            let y = -Self::get_heart_y(t) * 8.0;
            self.particles
                .push(Particle::new(x, y, true, 4.0, &mut self.rng));
        }

        // Create fill particles
        let spacing = 12.0;
        let mut attempts = 0;
        let max_attempts = 1000;

        while attempts < max_attempts {
            let t = self.rng.gen::<f32>() * 2.0 * PI;
            let scale = self.rng.gen_range(0.1..0.9);

            let x = Self::get_heart_x(t) * 8.0 * scale;
            let y = -Self::get_heart_y(t) * 8.0 * scale;

            if Self::is_inside_heart(x, y) {
                let mut too_close = false;
                for p in &self.particles {
                    let dx = x - p.target_x;
                    let dy = y - p.target_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < spacing {
                        too_close = true;
                        break;
                    }
                }

                if !too_close {
                    self.particles
                        .push(Particle::new(x, y, false, 4.0, &mut self.rng));
                }
            }
            attempts += 1;
        }

        // Add extra points for the bottom region
        for _ in 0..5 {
            let x = self.rng.gen_range(-10.0..10.0);
            let y = self.rng.gen_range(80.0..100.0);

            if Self::is_inside_heart(x, y) {
                let mut too_close = false;
                for p in &self.particles {
                    let dx = x - p.target_x;
                    let dy = y - p.target_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < spacing * 0.8 {
                        too_close = true;
                        break;
                    }
                }

                if !too_close {
                    self.particles
                        .push(Particle::new(x, y, false, 4.0, &mut self.rng));
                }
            }
        }
    }

    fn update(&mut self, timestamp: f64) {
        let frame_count = ((timestamp - self.start_time) / 16.67) as u32; // ~60fps

        match self.phase {
            Phase::Initializing => {
                let mut particles_in_position = 0;
                for p in &self.particles {
                    if p.is_near_target() {
                        particles_in_position += 1;
                    }
                }

                if particles_in_position as f32 > self.particles.len() as f32 * 0.95 {
                    self.phase = Phase::Formed;
                }
            }
            Phase::Formed => {
                let beat_frames = (60.0 * 60.0 / self.heart_rate) as f64;
                if timestamp - self.last_beat > beat_frames {
                    self.phase = Phase::Exploding;
                    for p in &mut self.particles {
                        p.explode(&mut self.rng);
                    }
                    self.last_beat = timestamp;
                }
            }
            Phase::Exploding => {
                if timestamp - self.last_beat > 20.0 * 16.67 {
                    self.phase = Phase::Reforming;
                }
            }
            Phase::Reforming => {
                let mut particles_reformed = 0;
                for p in &self.particles {
                    if p.is_near_target() {
                        particles_reformed += 1;
                    }
                }

                if particles_reformed as f32 > self.particles.len() as f32 * 0.95 {
                    self.phase = Phase::Formed;
                }
            }
        }

        // Update particles
        for p in &mut self.particles {
            p.update(self.phase);
        }
    }

    fn render(&self) {
        let gl = &self.gl;

        // Clear canvas
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        // Use shader program
        gl.use_program(Some(&self.program));

        // Set viewport transform - flip y to match canvas coordinate system
        let u_transform = gl
            .get_uniform_location(&self.program, "uTransform")
            .unwrap();
        gl.uniform_matrix4fv_with_f32_array(
            Some(&u_transform),
            false,
            &[
                2.0 / self.canvas_width,
                0.0,
                0.0,
                0.0,
                0.0,
                -2.0 / self.canvas_height,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        );

        // Bind vertex buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));

        // Enable attributes
        let position_loc = gl.get_attrib_location(&self.program, "aPosition") as u32;
        let point_size_loc = gl.get_attrib_location(&self.program, "aPointSize") as u32;

        gl.enable_vertex_attrib_array(position_loc);
        gl.enable_vertex_attrib_array(point_size_loc);

        // Set attribute pointers
        gl.vertex_attrib_pointer_with_i32(position_loc, 2, GL::FLOAT, false, 28, 0);
        gl.vertex_attrib_pointer_with_i32(point_size_loc, 1, GL::FLOAT, false, 28, 24);

        // Create vertex data for all particles
        let mut vertex_data = Vec::with_capacity(self.particles.len() * 7);

        for p in &self.particles {
            // Translate to center of canvas
            let x = p.x;
            let y = p.y;

            // Position (x, y)
            vertex_data.push(x);
            vertex_data.push(y);

            // Color (r, g, b, a)
            vertex_data.push(1.0); // r
            vertex_data.push(0.0); // g
            vertex_data.push(0.39); // b
            vertex_data.push(p.alpha / 255.0); // a

            // Point size
            vertex_data.push(p.size);
        }

        // Upload vertex data
        let vertices_array = Float32Array::from(&vertex_data[..]);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::DYNAMIC_DRAW);

        // Draw particles
        gl.draw_arrays(GL::POINTS, 0, self.particles.len() as i32);

        // Render text (we'll use HTML for this)
    }
}

fn setup_shaders(gl: &GL) -> Result<WebGlProgram, JsValue> {
    // Vertex shader
    let vertex_shader_source = r#"#version 300 es
        in vec2 aPosition;
        in float aPointSize;
        uniform mat4 uTransform;        
        
        void main() {
            gl_Position = uTransform * vec4(aPosition, 0.0, 1.0);
            gl_PointSize = aPointSize;
        }
    "#;

    // Fragment shader
    let fragment_shader_source = r#"#version 300 es
        precision mediump float;
        out vec4 outColor;
        uniform vec3 color_multiplier;

        void main() {
            // Create circular points
            vec2 center = vec2(0.5, 0.5);
            outColor = vec4(
                color_multiplier.r,
                color_multiplier.g,
                color_multiplier.b,
                1.0
            );
            float dist = distance(gl_PointCoord, center);
            if (dist > 0.5) {
                discard;
            }
        }
    "#;

    let vert_shader = match compile_shader(gl, GL::VERTEX_SHADER, vertex_shader_source) {
        Ok(shader) => shader,
        Err(e) => {
            console_error!("Vertex shader compilation failed: {}", e);
            return Err(JsValue::from_str(&e));
        }
    };

    let frag_shader = match compile_shader(gl, GL::FRAGMENT_SHADER, fragment_shader_source) {
        Ok(shader) => shader,
        Err(e) => {
            console_error!("Fragment shader compilation failed: {}", e);
            return Err(JsValue::from_str(&e));
        }
    };

    let program = gl
        .create_program()
        .ok_or("Unable to create shader program")?;
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    // Check for errors
    if !gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let error = gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string());
        return Err(JsValue::from_str(&error));
    }

    Ok(program)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

// Animation controller
#[wasm_bindgen]
pub struct HeartController {
    animation: Rc<RefCell<HeartAnimation>>,
    performance: Performance,
}

#[wasm_bindgen]
impl HeartController {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<HeartController, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;

        canvas.set_width(width);
        canvas.set_height(height);

        // Get WebGL context
        let gl = canvas.get_context("webgl2")?.unwrap().dyn_into::<GL>()?;

        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        // Create animation
        let animation = HeartAnimation::new(gl, width, height)?;
        let animation = Rc::new(RefCell::new(animation));

        let performance = window().unwrap().performance().unwrap();

        Ok(HeartController {
            animation,
            performance,
        })
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        let timestamp = self.performance.now();

        {
            let mut animation = self.animation.borrow_mut();
            animation.update(timestamp);
        }

        {
            let animation = self.animation.borrow();
            animation.render();
        }

        Ok(())
    }
    pub fn set_color(&self, r: f32, g: f32, b: f32) {
        let gl = &self.animation.borrow().gl;
        match gl.get_uniform_location(&self.animation.borrow().program, "color_multiplier") {
            Some(color_location) => {
                gl.uniform3f(Some(&color_location), r, g, b);
            }
            None => {
                console_error!("color_multiplier uniform not found")
            }
        };
    }

    pub fn stop(&mut self) {}

    #[wasm_bindgen]
    pub fn destroy(&mut self) {
        self.stop();
    }
}

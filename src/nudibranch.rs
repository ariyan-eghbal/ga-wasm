use crate::helpers::*;
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram,
    WebGlVertexArrayObject,
};

#[allow(unused)]
#[wasm_bindgen]
pub struct Nudibranch {
    gl: GL,
    program: WebGlProgram,
    vertex_array: WebGlVertexArrayObject,
    buffer: WebGlBuffer,
    time: f64,
    width: u32,
    height: u32,
    points_count: i32,
}

#[wasm_bindgen]
impl Nudibranch {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<Self, JsValue> {
        let document = web_sys::window()
            .ok_or_else(|| {
                console_error!("No window found");
                "No window found"
            })?
            .document()
            .ok_or_else(|| {
                console_error!("No document found");
                "No document found"
            })?;
        let canvas = document
            .get_element_by_id("canvas")
            .ok_or_else(|| {
                console_error!("Canvas not found");
                "Canvas not found"
            })?
            .dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(width);
        canvas.set_height(height);

        let gl = canvas
            .get_context("webgl2")?
            .ok_or_else(|| {
                console_error!("Failed to get WebGL context");
                "WebGL context creation failed"
            })?
            .dyn_into::<GL>()?;
        let vertex_shader = match compile_shader(
            &gl,
            GL::VERTEX_SHADER,
            r#"#version 300 es
            in vec2 position;
            uniform float time;
            uniform vec2 resolution;
            out float v_stroke;
            
            void main() {
                float x = position.x ;
                float y = position.y ;
                
                float k = (x/8.0 - 12.0);
                float e = (y/13.0 - 14.0);
                float o = sqrt(k*k + e*e)/2.0;
                float d = 5.0 * cos(o);
                
                float q = (x/2.0 + 10.0 + 1.0/k + k*cos(e)*sin(d*8.0 - time));
                float c = d/3.0 + time/8.0;
                
                float newX = q*sin(c) + sin(d*2.0 + time)*k + 200.0;
                float newY = ((y/4.0 + 5.0*o*o + q*cos(c*3.0))/2.0)*cos(c) + 200.0;
                
                vec2 finalPosition = vec2(
                    (newX / resolution.x) * 2.0 - 1.0,
                    (newY / resolution.y) * 2.0 - 1.0
                );
                
                gl_Position = vec4(finalPosition, 0.0, 1.0);
                gl_PointSize = 1.0;
            }
            "#,
        ) {
            Ok(shader) => shader,
            Err(e) => {
                console_error!("Vertex shader compilation failed: {}", e);
                return Err(JsValue::from_str(&e));
            }
        };

        let fragment_shader = match compile_shader(
            &gl,
            GL::FRAGMENT_SHADER,
            r#"#version 300 es
            precision highp float;
            uniform vec3 color_multiplier;
            out vec4 outColor;
            
            void main() {
                outColor = vec4(
                    color_multiplier.r,
                    color_multiplier.g,
                    color_multiplier.b,
                    1.0
                );
            }
            "#,
        ) {
            Ok(shader) => shader,
            Err(e) => {
                console_error!("Fragment shader compilation failed: {}", e);
                return Err(JsValue::from_str(&e));
            }
        };

        let program = match link_program(&gl, &vertex_shader, &fragment_shader) {
            Ok(prog) => prog,
            Err(e) => {
                console_error!("Program linking failed: {}", e);
                return Err(JsValue::from_str(&e));
            }
        };
        gl.use_program(Some(&program));

        let mut vertices: Vec<f32> = Vec::with_capacity(20000 * 2);
        for i in 0..20000 {
            let x = (i % 200) as f32;
            let y = (i / 200) as f32;
            vertices.push(x);
            vertices.push(y);
            vertices.push(i as f32);
        }

        let vertex_array = gl.create_vertex_array().ok_or_else(|| {
            console_error!("Failed to vertex array");
            "Failed to create vertex array"
        })?;
        gl.bind_vertex_array(Some(&vertex_array));

        let buffer = gl.create_buffer().ok_or_else(|| {
            console_error!("Failed to create buffer");
            "Failed to create buffer"
        })?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        let position_attribute_location = gl.get_attrib_location(&program, "position") as u32;

        gl.enable_vertex_attrib_array(position_attribute_location);
        gl.vertex_attrib_pointer_with_i32(position_attribute_location, 2, GL::FLOAT, false, 12, 0);

        Ok(Self {
            points_count: 20000,
            time: 0.0,
            gl,
            program,
            vertex_array,
            buffer,
            width,
            height,
        })
    }

    #[wasm_bindgen]
    pub fn set_color(&self, r: f32, g: f32, b: f32) {
        let gl = &self.gl;
        match gl.get_uniform_location(&self.program, "color_multiplier") {
            Some(color_location) => {
                gl.uniform3f(Some(&color_location), r, g, b);
            }
            None => {
                console_error!("color_multiplier uniform not found")
            }
        };
    }

    pub fn draw(&mut self) {
        let gl = &self.gl;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        let time_location = gl
            .get_uniform_location(&self.program, "time")
            .ok_or_else(|| {
                console_error!("time uniform not found");
            })
            .expect("Failed to create buffer");

        match gl.get_uniform_location(&self.program, "resolution") {
            Some(resolution_location) => {
                gl.uniform1f(Some(&time_location), self.time as f32);
                gl.uniform2f(
                    Some(&resolution_location),
                    self.width as f32,
                    self.height as f32,
                );

                gl.bind_vertex_array(Some(&self.vertex_array));
                gl.draw_arrays(GL::POINTS, 0, self.points_count);

                self.time += std::f64::consts::PI / 60.0;
            }
            None => {
                console_error!("resolution uniform not found1");
            }
        }
    }
    pub fn stop(&mut self) {}

    #[wasm_bindgen]
    pub fn destroy(&mut self) {
        self.stop();
    }
}

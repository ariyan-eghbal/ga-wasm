use crate::helpers::*;
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram,
    WebGlVertexArrayObject,
};

#[allow(unused)]
#[wasm_bindgen]
pub struct ChristmasTree {
    gl: GL,
    program: WebGlProgram,
    vertex_array: WebGlVertexArrayObject,
    buffer: WebGlBuffer,
    time: f64,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl ChristmasTree {
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

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
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
            uniform float time;
            uniform vec2 resolution;
            uniform vec3 color_multiplier;
            out vec4 outColor;

            void main() {
                vec2 r = resolution;
                float t = time;
                vec2 FC = gl_FragCoord.xy;
                vec4 o = vec4(0.0);

                float i = 0.0;
                float e, y;

                for(; i < 270.0; i++) {
                    y = i * 0.0015;
                    float a = i * 0.5 + t;
                    float s = floor(y / 0.05);
                    vec2 u = (FC.xy - 0.5 * r) / r.y * 0.5;

                    if(y > 0.26) {
                        e = length(u - vec2(0, y - 0.37)) + abs(u.x);
                    } else {
                        e = length(u - vec2(sin(a) * (0.03 + s * 0.025 - mod(y, 0.03)), 0.2 - y));
                    }

                    vec4 color;
                    if(y > 9.0) {
                        color = vec4(0.7, 0.5, 0.5, 0);
                    } else {
                        color = (0.2 + 0.17 * cos(i * 0.1 + t + vec4(0, 1, 2, 0)));
                    }

                    o += color * 2e-5 / (1e-9 + e * e);
                }

                outColor = vec4(o.rgb * color_multiplier, o.a);
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

        // Full-screen quad vertices
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];

        let vertex_array = gl.create_vertex_array().ok_or_else(|| {
            console_error!("Failed to create vertex array");
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
        gl.vertex_attrib_pointer_with_i32(position_attribute_location, 2, GL::FLOAT, false, 8, 0);

        Ok(Self {
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
            .expect("Failed to get time uniform");

        match gl.get_uniform_location(&self.program, "resolution") {
            Some(resolution_location) => {
                gl.uniform1f(Some(&time_location), self.time as f32);
                gl.uniform2f(
                    Some(&resolution_location),
                    self.width as f32,
                    self.height as f32,
                );

                gl.bind_vertex_array(Some(&self.vertex_array));
                gl.draw_arrays(GL::TRIANGLES, 0, 6);

                self.time += std::f64::consts::PI / 120.0;
            }
            None => {
                console_error!("resolution uniform not found");
            }
        }
    }

    pub fn stop(&mut self) {}

    #[wasm_bindgen]
    pub fn destroy(&mut self) {
        self.stop();
    }
}

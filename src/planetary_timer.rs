use crate::helpers::*;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram};

#[wasm_bindgen]
pub struct PlanetaryTimer {
    gl: GL,
    program: WebGlProgram,
    time: f32,
    width: f32,
    height: f32,
}

#[wasm_bindgen]
impl PlanetaryTimer {
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

        let gl = canvas.get_context("webgl2")?.unwrap().dyn_into::<GL>()?;

        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        let program = create_shader_program(&gl)?;

        Ok(PlanetaryTimer {
            gl,
            program,
            time: 0.0,
            width,
            height,
        })
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.gl.viewport(0, 0, width as i32, height as i32);
    }

    #[wasm_bindgen]
    pub fn draw(&mut self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl.use_program(Some(&self.program));

        let time_location = self
            .gl
            .get_uniform_location(&self.program, "time")
            .ok_or_else(|| {
                console_error!("time uniform not found");
            })
            .expect("Failed to get time uniform");

        let resolution_location = self
            .gl
            .get_uniform_location(&self.program, "resolution")
            .ok_or_else(|| {
                console_error!("resolution uniform not found");
            })
            .expect("Failed to get resolution uniform");

        self.gl.uniform1f(Some(&time_location), self.time);
        self.gl
            .uniform2f(Some(&resolution_location), self.width, self.height);

        self.gl.draw_arrays(GL::TRIANGLE_STRIP, 0, 4);

        self.time += PI / 60.0;
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

    pub fn stop(&mut self) {}

    #[wasm_bindgen]
    pub fn destroy(&mut self) {
        self.stop();
    }
}

fn create_shader_program(gl: &GL) -> Result<WebGlProgram, JsValue> {
    let vertex_shader_source = r#"#version 300 es
    in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }"#;

    let fragment_shader_source = r#"#version 300 es
    precision highp float;
    uniform float time;
    uniform vec2 resolution;
    uniform vec3 color_multiplier;
    out vec4 fragColor;

    void main() {
        vec2 r = resolution;
        vec2 FC = gl_FragCoord.xy;
        vec2 p = (FC.xy * 2.0 - r) / r.x * 0.25;
        
        vec3 C = vec3(0.0);
        float t = time;
        
        for(float i = 1.0; i < 99.0; i++) {
            float j = i;
            vec2 q = p - vec2(sin(5.0 + cos(t * 0.5) + sin(t * 0.5) / j * 99.0) * 0.4,
                            sin(t * 0.5 - j)) * 0.1;
            C += 0.0025 / length(q * 5.0);
        }
        
        // vec4 o = vec4(0.0);
        vec4 o = vec4(
                    color_multiplier.r,
                    color_multiplier.g,
                    color_multiplier.b,
                    1.0
                );
        o += vec4(C, 0.0) - 0.008 / (length(p) - 0.108);
        o += -1.2;
        
        fragColor = vec4(max(o.rgb, 0.0), 1.0);
    }"#;

    let vertex_shader = compile_shader(gl, GL::VERTEX_SHADER, vertex_shader_source)
        .map_err(|e| JsValue::from_str(&e))?;

    let fragment_shader = compile_shader(gl, GL::FRAGMENT_SHADER, fragment_shader_source)
        .map_err(|e| JsValue::from_str(&e))?;

    let program =
        link_program(gl, &vertex_shader, &fragment_shader).map_err(|e| JsValue::from_str(&e))?;

    let vertices: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];

    let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vertices_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);
    }

    let position_attr = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(
        position_attr,
        2,         // 2 components per vertex
        GL::FLOAT, // data type
        false,     // normalized
        0,         // stride
        0,         // offset
    );
    gl.enable_vertex_attrib_array(position_attr);

    Ok(program)
}

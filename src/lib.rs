use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram, WebGlShader,
    WebGlVertexArrayObject,
};

#[allow(unused)]
#[wasm_bindgen]
pub struct GenerativeArt {
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
impl GenerativeArt {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<GenerativeArt, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;

        canvas.set_width(width);
        canvas.set_height(height);

        let gl = canvas.get_context("webgl2")?.unwrap().dyn_into::<GL>()?;

        let vertex_shader = compile_shader(
            &gl,
            GL::VERTEX_SHADER,
            r#"#version 300 es
            in vec2 position;
            in float index;
            uniform float time;
            uniform vec2 resolution;
            out float v_stroke;
            
            float mag(float k, float e) {
                return sqrt(k * k + e * e);
            }
            
            void main() {
                float x = position.x;
                float y = position.y;
                
                float k = x/8.0 - 12.5;
                float e = y/8.0 - 12.5;
                float o = pow(mag(k, e), 2.0)/169.0;
                float d = 0.5 + 5.0*cos(o);
                
                v_stroke = pow(d * sin(k) * sin(time * 4.0 + e), 2.0);
                
                float new_x = x + d*k*sin(d*2.0 + o + time) + e*cos(e + time) + resolution.x/4.0;
                float new_y = o*135.0 - y/4.0 - d*6.0*cos(d*3.0 + o*9.0 + time) + resolution.y/3.2;
                
                gl_Position = vec4(new_x/(resolution.x/2.0) - 1.0, -(new_y/(resolution.y/2.0) - 1.0), 0, 1);
                gl_PointSize = 1.0;
            }
            "#,
        )?;

        let fragment_shader = compile_shader(
            &gl,
            GL::FRAGMENT_SHADER,
            r#"#version 300 es
            precision highp float;
            in float v_stroke;
            uniform vec3 color_multiplier;
            out vec4 outColor;
            
            void main() {
                outColor = vec4(
                    v_stroke * color_multiplier.r,
                    v_stroke * color_multiplier.g,
                    v_stroke * color_multiplier.b,
                    1
                );
            }
            "#,
        )?;

        let program = link_program(&gl, &vertex_shader, &fragment_shader)?;
        gl.use_program(Some(&program));

        let mut vertices: Vec<f32> = Vec::with_capacity(40000 * 3);
        for i in 0..40000 {
            let x = (i % 200) as f32;
            let y = (i / 200) as f32;
            vertices.push(x);
            vertices.push(y);
            vertices.push(i as f32);
        }

        let vertex_array = gl
            .create_vertex_array()
            .ok_or("Failed to create vertex array")?;
        gl.bind_vertex_array(Some(&vertex_array));

        let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        let position_attribute_location = gl.get_attrib_location(&program, "position") as u32;
        let index_attribute_location = gl.get_attrib_location(&program, "index") as u32;

        gl.enable_vertex_attrib_array(position_attribute_location);
        gl.vertex_attrib_pointer_with_i32(position_attribute_location, 2, GL::FLOAT, false, 12, 0);

        gl.enable_vertex_attrib_array(index_attribute_location);
        gl.vertex_attrib_pointer_with_i32(index_attribute_location, 1, GL::FLOAT, false, 12, 8);

        Ok(GenerativeArt {
            points_count: 40000,
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
        let color_location = gl
            .get_uniform_location(&self.program, "color_multiplier")
            .expect("Color multiplier uniform not found");
        gl.uniform3f(Some(&color_location), r, g, b);
    }

    pub fn draw(&mut self) {
        let gl = &self.gl;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        // Update uniforms
        let time_location = gl
            .get_uniform_location(&self.program, "time")
            .expect("Time uniform not found");
        let resolution_location = gl
            .get_uniform_location(&self.program, "resolution")
            .expect("Resolution uniform not found");

        gl.uniform1f(Some(&time_location), self.time as f32);
        gl.uniform2f(
            Some(&resolution_location),
            self.width as f32,
            self.height as f32,
        );

        gl.bind_vertex_array(Some(&self.vertex_array));
        gl.draw_arrays(GL::POINTS, 0, self.points_count);

        self.time += std::f64::consts::PI / 120.0;
    }
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("error creating shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("error creating shader")))
    }
}

fn link_program(
    gl: &GL,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("error creating shader instance"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("error creating object")))
    }
}

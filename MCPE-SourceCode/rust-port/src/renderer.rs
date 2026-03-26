use glow::HasContext;
use std::rc::Rc;

pub struct GameRenderer {
    pub gl: Option<Rc<glow::Context>>,
    program: Option<glow::NativeProgram>,
    vao: Option<glow::NativeVertexArray>,
    vbo: Option<glow::NativeBuffer>,
    mesh_vao: Option<glow::NativeVertexArray>,
    mesh_vbo: Option<glow::NativeBuffer>,
    mesh_vertex_count: i32,
    texture: Option<glow::NativeTexture>,
}

impl GameRenderer {
    pub fn new(gl: Option<Rc<glow::Context>>) -> Self {
        let mut out = Self {
            gl: gl.clone(),
            program: None,
            vao: None,
            vbo: None,
            mesh_vao: None,
            mesh_vbo: None,
            mesh_vertex_count: 0,
        };

        if let Some(gl) = &gl {
            unsafe {
                // Simple vertex + fragment shader (with UV + texture)
                let vs_src = r#"
                    #version 330 core
                    layout(location = 0) in vec3 aPos;
                    layout(location = 1) in vec3 aColor;
                    layout(location = 2) in vec2 aUV;
                    uniform mat4 u_proj;
                    uniform vec3 u_cam_pos;
                    uniform vec3 u_offset;
                    out vec3 vColor;
                    out vec2 vUV;
                    void main() {
                        vColor = aColor;
                        vUV = aUV;
                        vec3 pos = aPos + u_offset - u_cam_pos;
                        gl_Position = u_proj * vec4(pos, 1.0);
                    }
                "#;

                let fs_src = r#"
                    #version 330 core
                    in vec3 vColor;
                    in vec2 vUV;
                    uniform sampler2D u_texture;
                    out vec4 FragColor;
                    void main() {
                        vec4 tex = texture(u_texture, vUV);
                        FragColor = tex * vec4(vColor, 1.0);
                    }
                "#;

                let compile = |gl: &glow::Context, src: &str, ty: u32| {
                    let shader = gl.create_shader(ty).unwrap();
                    gl.shader_source(shader, src);
                    gl.compile_shader(shader);
                    if !gl.get_shader_compile_status(shader) {
                        let log = gl.get_shader_info_log(shader);
                        gl.delete_shader(shader);
                        panic!("Shader compile error: {}", log);
                    }
                    shader
                };

                let vs = compile(&gl, vs_src, glow::VERTEX_SHADER);
                let fs = compile(&gl, fs_src, glow::FRAGMENT_SHADER);

                let program = gl.create_program().expect("create program");
                gl.attach_shader(program, vs);
                gl.attach_shader(program, fs);
                gl.link_program(program);
                if !gl.get_program_link_status(program) {
                    let log = gl.get_program_info_log(program);
                    panic!("Program link error: {}", log);
                }
                gl.delete_shader(vs);
                gl.delete_shader(fs);

                let base6: [f32; 36 * 6] = [
                    // positions        // colors
                    -0.5, -0.5, -0.5, 1.0, 0.0, 0.0,
                    0.5, -0.5, -0.5, 0.0, 1.0, 0.0,
                    0.5,  0.5, -0.5, 0.0, 0.0, 1.0,
                    0.5,  0.5, -0.5, 0.0, 0.0, 1.0,
                    -0.5,  0.5, -0.5, 1.0, 1.0, 0.0,
                    -0.5, -0.5, -0.5, 1.0, 0.0, 0.0,

                    -0.5, -0.5,  0.5, 1.0, 0.0, 1.0,
                    0.5, -0.5,  0.5, 0.0, 1.0, 1.0,
                    0.5,  0.5,  0.5, 1.0, 1.0, 1.0,
                    0.5,  0.5,  0.5, 1.0, 1.0, 1.0,
                    -0.5,  0.5,  0.5, 0.5, 0.5, 0.5,
                    -0.5, -0.5,  0.5, 1.0, 0.0, 1.0,

                    -0.5,  0.5,  0.5, 0.2, 0.7, 0.3,
                    -0.5,  0.5, -0.5, 0.9, 0.3, 0.2,
                    -0.5, -0.5, -0.5, 0.4, 0.6, 0.8,
                    -0.5, -0.5, -0.5, 0.4, 0.6, 0.8,
                    -0.5, -0.5,  0.5, 0.1, 0.2, 0.9,
                    -0.5,  0.5,  0.5, 0.2, 0.7, 0.3,

                    0.5,  0.5,  0.5, 0.6, 0.4, 0.2,
                    0.5,  0.5, -0.5, 0.3, 0.6, 0.9,
                    0.5, -0.5, -0.5, 0.7, 0.2, 0.3,
                    0.5, -0.5, -0.5, 0.7, 0.2, 0.3,
                    0.5, -0.5,  0.5, 0.2, 0.8, 0.4,
                    0.5,  0.5,  0.5, 0.6, 0.4, 0.2,

                    -0.5, -0.5, -0.5, 0.3, 0.3, 0.7,
                    0.5, -0.5, -0.5, 0.2, 0.4, 0.2,
                    0.5, -0.5,  0.5, 0.4, 0.2, 0.6,
                    0.5, -0.5,  0.5, 0.4, 0.2, 0.6,
                    -0.5, -0.5,  0.5, 0.7, 0.7, 0.2,
                    -0.5, -0.5, -0.5, 0.3, 0.3, 0.7,

                    -0.5,  0.5, -0.5, 0.6, 0.1, 0.6,
                    0.5,  0.5, -0.5, 0.1, 0.6, 0.3,
                    0.5,  0.5,  0.5, 0.2, 0.3, 0.8,
                    0.5,  0.5,  0.5, 0.2, 0.3, 0.8,
                    -0.5,  0.5,  0.5, 0.9, 0.9, 0.1,
                    -0.5,  0.5, -0.5, 0.6, 0.1, 0.6,
                ];

                let mut vertices: Vec<f32> = Vec::with_capacity(36 * 8);
                for i in 0..36 {
                    let base = i * 6;
                    vertices.push(base6[base + 0]);
                    vertices.push(base6[base + 1]);
                    vertices.push(base6[base + 2]);
                    vertices.push(base6[base + 3]);
                    vertices.push(base6[base + 4]);
                    vertices.push(base6[base + 5]);
                    // default UVs
                    vertices.push(0.0);
                    vertices.push(0.0);
                }

                let vao = gl.create_vertex_array().unwrap();
                let vbo = gl.create_buffer().unwrap();

                gl.bind_vertex_array(Some(vao));
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
                let data_u8 = std::slice::from_raw_parts(
                    vertices.as_ptr() as *const u8,
                    vertices.len() * std::mem::size_of::<f32>(),
                );
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data_u8, glow::STATIC_DRAW);

                gl.enable_vertex_attrib_array(0);
                gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 8 * 4, 0);
                gl.enable_vertex_attrib_array(1);
                gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 8 * 4, 3 * 4);
                gl.enable_vertex_attrib_array(2);
                gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 8 * 4, 6 * 4);

                gl.bind_buffer(glow::ARRAY_BUFFER, None);
                gl.bind_vertex_array(None);

                // create a tiny 1x1 white texture as placeholder
                let tex = gl.create_texture().unwrap();
                gl.bind_texture(glow::TEXTURE_2D, Some(tex));
                let white: [u8; 4] = [255, 255, 255, 255];
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA as i32,
                    1,
                    1,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    Some(&white),
                );
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
                gl.bind_texture(glow::TEXTURE_2D, None);

                // Bind sampler uniform to texture unit 0
                let loc_tex = gl.get_uniform_location(program, "u_texture");
                gl.use_program(Some(program));
                gl.uniform_1_i32(loc_tex.as_ref(), 0);
                gl.use_program(None);

                out.program = Some(program);
                out.vao = Some(vao);
                out.vbo = Some(vbo);
            }
        }

        out
    }

    pub fn set_viewport(&self, width: i32, height: i32) {
        if let Some(gl) = &self.gl {
            unsafe {
                gl.viewport(0, 0, width, height);
            }
        }
    }

    pub fn render(&self, proj: &[f32; 16], cam_pos: [f32; 3], offset: [f32; 3]) {
        if let Some(gl) = &self.gl {
            unsafe {
                gl.clear_color(0.1, 0.2, 0.3, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                gl.enable(glow::DEPTH_TEST);

                if let Some(program) = self.program {
                    gl.use_program(Some(program));
                    let loc_proj = gl.get_uniform_location(program, "u_proj");
                    if let Some(loc) = loc_proj.as_ref() {
                        gl.uniform_matrix_4_f32_slice(Some(loc), false, proj);
                    }
                    let loc_cam = gl.get_uniform_location(program, "u_cam_pos");
                    gl.uniform_3_f32(loc_cam.as_ref(), cam_pos[0], cam_pos[1], cam_pos[2]);
                    let loc_offset = gl.get_uniform_location(program, "u_offset");
                    gl.uniform_3_f32(loc_offset.as_ref(), offset[0], offset[1], offset[2]);

                    if let Some(vao) = self.vao {
                        gl.bind_vertex_array(Some(vao));
                        gl.draw_arrays(glow::TRIANGLES, 0, 36);
                        gl.bind_vertex_array(None);
                    }

                    gl.use_program(None);
                }
            }
        }
    }

    pub fn upload_mesh(&mut self, vertices: &[f32]) {
        if let Some(gl) = &self.gl {
            unsafe {
                // Delete old mesh buffers if any
                if let Some(old_vbo) = self.mesh_vbo.take() {
                    gl.delete_buffer(old_vbo);
                }
                if let Some(old_vao) = self.mesh_vao.take() {
                    gl.delete_vertex_array(old_vao);
                }

                let vao = gl.create_vertex_array().unwrap();
                let vbo = gl.create_buffer().unwrap();

                gl.bind_vertex_array(Some(vao));
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

                let data_u8 = std::slice::from_raw_parts(
                    vertices.as_ptr() as *const u8,
                    vertices.len() * std::mem::size_of::<f32>(),
                );
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data_u8, glow::STATIC_DRAW);

                gl.enable_vertex_attrib_array(0);
                gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 8 * 4, 0);
                gl.enable_vertex_attrib_array(1);
                gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 8 * 4, 3 * 4);
                gl.enable_vertex_attrib_array(2);
                gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 8 * 4, 6 * 4);

                gl.bind_buffer(glow::ARRAY_BUFFER, None);
                gl.bind_vertex_array(None);

                self.mesh_vao = Some(vao);
                self.mesh_vbo = Some(vbo);
                self.mesh_vertex_count = (vertices.len() / 8) as i32;
            }
        }
    }

    pub fn render_scene(&self, proj: &[f32; 16], cam_pos: [f32; 3]) {
        if let Some(gl) = &self.gl {
            unsafe {
                gl.clear_color(0.1, 0.2, 0.3, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                gl.enable(glow::DEPTH_TEST);

                if let Some(program) = self.program {
                    gl.use_program(Some(program));
                    let loc_proj = gl.get_uniform_location(program, "u_proj");
                    if let Some(loc) = loc_proj.as_ref() {
                        gl.uniform_matrix_4_f32_slice(Some(loc), false, proj);
                    }
                    let loc_cam = gl.get_uniform_location(program, "u_cam_pos");
                    gl.uniform_3_f32(loc_cam.as_ref(), cam_pos[0], cam_pos[1], cam_pos[2]);
                    let loc_offset = gl.get_uniform_location(program, "u_offset");
                    gl.uniform_3_f32(loc_offset.as_ref(), 0.0, 0.0, 0.0);

                    // Bind texture unit 0
                    gl.active_texture(glow::TEXTURE0);
                    if let Some(tex) = self.texture {
                        gl.bind_texture(glow::TEXTURE_2D, Some(tex));
                    }

                    if let Some(vao) = self.mesh_vao {
                        gl.bind_vertex_array(Some(vao));
                        gl.draw_arrays(glow::TRIANGLES, 0, self.mesh_vertex_count);
                        gl.bind_vertex_array(None);
                    } else if let Some(vao) = self.vao {
                        gl.bind_vertex_array(Some(vao));
                        gl.draw_arrays(glow::TRIANGLES, 0, 36);
                        gl.bind_vertex_array(None);
                    }

                    // Unbind texture
                    gl.bind_texture(glow::TEXTURE_2D, None);

                    gl.use_program(None);
                }
            }
        }
    }
}
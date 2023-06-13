pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vert_path: &str, frag_path: &str) -> Self {
        let vert_shader: u32;
        let frag_shader: u32;
        unsafe {
            vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
            frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        }
        // load shader from string
        unsafe {
            let shader_source = std::fs::read_to_string(vert_path)
                .expect(std::format!("Cannot read {vert_path}").as_str());

            gl::ShaderSource(
                vert_shader,
                1,
                &(shader_source.as_str().as_bytes().as_ptr().cast()),
                &(shader_source.len().try_into().unwrap()),
            );
            gl::CompileShader(vert_shader);
        }
        check_for_error(&vert_shader, "VERTEX");

        unsafe {
            let shader_source = std::fs::read_to_string(frag_path)
                .expect(std::format!("Cannot read {frag_path}").as_str());
            gl::ShaderSource(
                frag_shader,
                1,
                &(shader_source.as_str().as_bytes().as_ptr().cast()),
                &(shader_source.len().try_into().unwrap()),
            );
            gl::CompileShader(frag_shader);
        }
        check_for_error(&frag_shader, "FRAGMENT");

        let shader_program;
        unsafe {
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vert_shader);
            gl::AttachShader(shader_program, frag_shader);
            gl::LinkProgram(shader_program);
        }
        check_for_error(&shader_program, "PROGRAM");

        unsafe {
            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);
        }

        Shader { id: shader_program }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4(&self, name: &str, value: &nalgebra_glm::Mat4) {
        let cname = std::ffi::CString::new(name).expect("CString::new failed.");
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, cname.as_ptr().cast()),
                1,
                gl::FALSE,
                value.as_ptr(),
            );
        }
    }

    pub fn set_i32(&self, name: &str, value: &i32) {
        let cname = std::ffi::CString::new(name).expect("CString::new failed.");
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, cname.as_ptr().cast()),
                *value,
            );
        }
    }
}

fn check_for_error(shader: &u32, shader_type: &str) {
    let mut success = 0;
    if shader_type.eq_ignore_ascii_case("PROGRAM") {
        unsafe {
            gl::GetProgramiv(*shader, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            unsafe {
                gl::GetProgramInfoLog(*shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }
            panic!(
                "Shader Program Linking Error: {}",
                String::from_utf8_lossy(&v)
            );
        }
    } else {
        unsafe {
            gl::GetShaderiv(*shader, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            unsafe {
                gl::GetShaderInfoLog(*shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }
            panic!(
                "Shader Compiling Error By {shader_type}: {}",
                String::from_utf8_lossy(&v)
            );
        }
    }
}

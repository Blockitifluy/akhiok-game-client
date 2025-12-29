use std::fs;

use ogl33::*;
use ultraviolet::Mat4;

#[macro_export]
macro_rules! null_str {
    ($lit:literal) => {{
        const _: &str = $lit;
        concat!($lit, "\0")
    }};
}

pub struct VertexArray(pub GLuint);
impl VertexArray {
    pub fn new() -> Option<Self> {
        let mut vao = 0_u32;
        unsafe {
            glGenVertexArrays(1, &mut vao);
        };
        if vao != 0 { Some(Self(vao)) } else { None }
    }

    pub fn bind(&self) {
        unsafe {
            glBindVertexArray(self.0);
        }
    }

    pub fn clear_binding() {
        unsafe {
            glBindVertexArray(0);
        }
    }
}

pub enum ShaderType {
    Vertex = GL_VERTEX_SHADER as isize,
    Fragment = GL_FRAGMENT_SHADER as isize,
}

pub struct Shader(pub GLuint);
impl Shader {
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { glCreateShader(ty as GLenum) };
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    pub fn set_source(&self, src: &str) {
        unsafe {
            glShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }

    pub fn compile(&self) {
        unsafe { glCompileShader(self.0) };
    }

    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) };
        compiled == i32::from(GL_TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn delete(self) {
        unsafe { glDeleteShader(self.0) };
    }
    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty).ok_or_else(|| "couldn't allocate new shader".to_string())?;
        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.delete();
            Err(out)
        }
    }
}

pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {
    pub fn new() -> Option<Self> {
        let prog = unsafe { glCreateProgram() };
        if prog != 0 { Some(Self(prog)) } else { None }
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            glAttachShader(self.0, shader.0);
        }
    }

    pub fn link_program(&self) {
        unsafe { glLinkProgram(self.0) };
    }

    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };
        success == i32::from(GL_TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe {
            glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len);
        };

        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn use_program(&self) {
        unsafe { glUseProgram(self.0) };
    }

    pub fn delete(self) {
        unsafe { glDeleteProgram(self.0) };
    }

    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let p = Self::new().ok_or_else(|| "couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("vertex compile error: {}", e))?;
        let f = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("fragment compile error: {}", e))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();
        v.delete();
        f.delete();
        if p.link_success() {
            Ok(p)
        } else {
            let out = format!("program link error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }

    pub fn from_vert_frag_file(vert_path: &str, frag_path: &str) -> Result<Self, String> {
        let (vert, frag) = (
            fs::read_to_string(vert_path).expect("couldn't read vert shader file"),
            fs::read_to_string(frag_path).expect("couldn't read frag shader file"),
        );

        Self::from_vert_frag(vert.as_str(), frag.as_str())
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            glUniform1i(
                glGetUniformLocation(self.0, name.as_ptr().cast()),
                value as i32,
            );
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            glUniform1i(glGetUniformLocation(self.0, name.as_ptr().cast()), value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            glUniform1f(glGetUniformLocation(self.0, name.as_ptr().cast()), value);
        }
    }

    pub fn set_matrix4(&self, name: &str, value: Mat4) {
        unsafe {
            glUniformMatrix4fv(
                glGetUniformLocation(self.0, name.as_ptr().cast()),
                1,
                GL_FALSE,
                value.as_ptr(),
            );
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Point = GL_POINT as isize,
    Line = GL_LINE as isize,
    Fill = GL_FILL as isize,
}

pub fn polygon_mode(mode: PolygonMode) {
    unsafe { glPolygonMode(GL_FRONT_AND_BACK, mode as GLenum) };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Array = GL_ARRAY_BUFFER as isize,
    ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);
impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 { Some(Self(vbo)) } else { None }
    }

    pub fn bind(&self, ty: BufferType) {
        unsafe { glBindBuffer(ty as GLenum, self.0) }
    }

    pub fn clear_binding(ty: BufferType) {
        unsafe { glBindBuffer(ty as GLenum, 0) }
    }
}

pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
        glBufferData(
            ty as GLenum,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        glClearColor(r, g, b, a);
    }
}

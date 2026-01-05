//! Adds many utility functions and types to help with rendering
use std::fs;

use ogl33::*;
use ultraviolet::Mat4;

use crate::datatypes::color::Color3;

/// A `vertex array object` used for rendering meshes.
pub struct VertexArray(pub GLuint);
impl VertexArray {
    /// Creates a new VAO
    /// # Returns
    /// Either:
    /// - `None`,
    /// - A new Vertex Array.
    pub fn new() -> Option<Self> {
        let mut vao = 0_u32;
        unsafe {
            glGenVertexArrays(1, &mut vao);
        };
        if vao != 0 { Some(Self(vao)) } else { None }
    }

    /// Binds the Vertex Array to GL.
    pub fn bind(&self) {
        unsafe {
            glBindVertexArray(self.0);
        }
    }

    /// Clear the Vertex Array binding to GL.
    pub fn clear_binding() {
        unsafe {
            glBindVertexArray(0);
        }
    }
}

/// The type of `Shader`
pub enum ShaderType {
    /// Vertex Shader
    Vertex = GL_VERTEX_SHADER as isize,
    /// Fragment Shader
    Fragment = GL_FRAGMENT_SHADER as isize,
}

/// A shader which could either be: `Vertex` or `Fragment`.
pub struct Shader(pub GLuint);
impl Shader {
    /// Creates a new Shader.
    /// # Arguements
    /// - `ty`: the shader type
    /// # Returns
    /// Either:
    /// - `None`,
    /// - A shader
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { glCreateShader(ty as GLenum) };
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    /// Sets the source code of the `shader`.
    /// # Arguements
    /// - `src`: the source code
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

    /// Compiles the source code to the `shader`.
    /// # Note
    /// The source code will need to be set using `set_source`.
    pub fn compile(&self) {
        unsafe { glCompileShader(self.0) };
    }

    /// Checks if the shader compilation has been successful
    /// # Notes
    /// - Run `compile` for this to work
    /// - To get the info logs
    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) };
        compiled == i32::from(GL_TRUE)
    }

    /// Return the error log when compiling the `shader`.
    /// # Returns
    /// The info log
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

    /// Deletes the `shader`
    pub fn delete(self) {
        unsafe { glDeleteShader(self.0) };
    }

    /// Creates and compiles a shader from it's type and source code.
    /// # Arguements
    /// - `ty`: the type of shader.
    /// - `source`: the source code of the shader (not the path).
    /// # Returns
    /// Either:
    /// - A shader,
    /// - An info log when an error occures from `info_log`
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

/// A program used in GL.
pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {
    /// Creates a new shader program.
    /// # Returns
    /// Either:
    /// - `None`: when the creation was unsuccessful
    /// - A new shader program
    pub fn new() -> Option<Self> {
        let prog = unsafe { glCreateProgram() };
        if prog != 0 { Some(Self(prog)) } else { None }
    }

    /// Attaches the shader to the shader program
    /// # Arguement
    /// - `shader`: the shader being attached
    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            glAttachShader(self.0, shader.0);
        }
    }

    /// Links the program to GL.
    pub fn link_program(&self) {
        unsafe { glLinkProgram(self.0) };
    }

    /// Gets the status of linking shaders to the program
    /// # Returns
    /// The error log
    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };
        success == i32::from(GL_TRUE)
    }

    /// Gets the info log when linking shaders into the program.
    /// # Returns
    /// The info log
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

    /// Uses the shader program in GL.
    pub fn use_program(&self) {
        unsafe { glUseProgram(self.0) };
    }

    /// Deletes the shader program.
    pub fn delete(self) {
        unsafe { glDeleteProgram(self.0) };
    }

    /// Creates a new program and links the fragmentation and vertex shader source code.
    /// # Arguements
    /// - `vert`: the vertex shader source code
    /// - `frag`: the fragmentation shader source code
    /// # Returns
    /// Either:
    /// - The shader program
    /// - An error when linking or compiling shader.
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

    /// Creates a new program and links the fragmentation and vertex shader source code from the files.
    /// # Arguements
    /// - `vert_path`: the vertex shader file path
    /// - `frag_path`: the fragmentation shader file path
    /// # Returns
    /// Either:
    /// - The shader program
    /// - An error when linking, opening files or compiling shaders.
    pub fn from_vert_frag_file(vert_path: &str, frag_path: &str) -> Result<Self, String> {
        let (vert, frag) = (
            fs::read_to_string(vert_path).expect("couldn't read vert shader file"),
            fs::read_to_string(frag_path).expect("couldn't read frag shader file"),
        );

        Self::from_vert_frag(vert.as_str(), frag.as_str())
    }

    /// Sets the a `bool` uniform value in the program.
    /// # Arguements
    /// - `name`: the name of the value
    /// - `value`: a boolean value
    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            glUniform1i(
                glGetUniformLocation(self.0, name.as_ptr().cast()),
                value as i32,
            );
        }
    }

    /// Sets the a `int` uniform value in the program.
    /// # Arguements
    /// - `name`: the name of the value
    /// - `value`: a integer value
    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            glUniform1i(glGetUniformLocation(self.0, name.as_ptr().cast()), value);
        }
    }

    /// Sets the a `float` uniform value in the program.
    /// # Arguements
    /// - `name`: the name of the value
    /// - `value`: a float value
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            glUniform1f(glGetUniformLocation(self.0, name.as_ptr().cast()), value);
        }
    }

    /// Sets the a `Mat4` uniform value in the program.
    /// # Arguements
    /// - `name`: the name of the value
    /// - `value`: a 4x4 Matrix value
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

/// The polygon that GL is rendering with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    /// GL_POINT
    Point = GL_POINT as isize,
    /// GL_LINE
    Line = GL_LINE as isize,
    /// GL_FILL
    Fill = GL_FILL as isize,
}

/// Set the `PolygonMode`.
/// # Arguements
/// - `mode`: the polygon mode
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { glPolygonMode(GL_FRONT_AND_BACK, mode as GLenum) };
}

/// The type of `Buffer` object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    /// GL_ARRAY_BUFFER
    Array = GL_ARRAY_BUFFER as isize,
    /// GL_ELEMENT_ARRAY_BUFFER
    ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

/// The buffer object used in GL rendering.
pub struct Buffer(pub GLuint);
impl Buffer {
    /// Creates a new buffer object
    /// # Returns
    /// Either:
    /// - `None` when creation was not successful,
    /// - A new buffer object
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 { Some(Self(vbo)) } else { None }
    }

    /// Binds the buffer
    /// # Arguements
    /// - `ty`: the type of the buffer
    pub fn bind(&self, ty: BufferType) {
        unsafe { glBindBuffer(ty as GLenum, self.0) }
    }

    /// Clears the buffer of a type
    /// # Arguements
    /// - `ty`: the type of buffer to clear
    pub fn clear_binding(ty: BufferType) {
        unsafe { glBindBuffer(ty as GLenum, 0) }
    }
}

/// Sets data inside a buffer
/// # Arguements
/// - `ty`: the type of buffer
/// - `data`: a byte array
/// - `usage`: How the buffer will be modified
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

/// Sets the clear color.
/// # Arguements
/// - `color`: the color
pub fn clear_color(color: Color3) {
    unsafe {
        glClearColor(color.r, color.g, color.b, 1.0);
    }
}

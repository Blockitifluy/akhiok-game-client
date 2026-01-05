//! Used for the `Window` helper structure. Containing various GL objects.

use std::ptr;

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlWindow},
    *,
};
use ogl33::*;
use ultraviolet::Mat4;

use crate::{gl_helper::*, mesh::Mesh};

/// A wrapper for `GlWindow`, shader program and multiple GL objects:
/// - `vao`,
/// - `vbo` and
/// - `ebo`
pub struct Window {
    /// Vertex Array Object
    pub vao: VertexArray,
    /// Vertex Buffer Object
    pub vbo: Buffer,
    /// Element Buffer Object
    pub ebo: Buffer,
    /// The shader program used in GL.
    pub shader_program: ShaderProgram,
    /// Simple DirectMedia Layer
    pub sdl: Sdl,
    /// The GL window
    pub window: GlWindow,
}
impl Window {
    /// Creates a new window, with Gl objects uninitilised.
    /// # Arguements
    /// - `args`: arguements to create the window
    /// # Returns
    /// The window. However can throw an error when it could create a window and context.
    pub fn new(args: CreateWinArgs) -> Result<Self, &'static str> {
        let sdl = Self::init_sdl();
        let win_ex = sdl.create_gl_window(args);

        let Ok(win) = win_ex else {
            return Err("couldn't make a window and context");
        };

        let win_struct = Self {
            window: win,
            sdl,
            shader_program: ShaderProgram(0),
            vao: VertexArray(0),
            vbo: Buffer(0),
            ebo: Buffer(0),
        };

        Ok(win_struct)
    }

    /// Initilises the objects and program for the window
    /// # Returns
    /// Nothing or an error message.
    pub fn init_objects(&mut self, vert: &str, frag: &str) -> Result<(), &'static str> {
        let vao_null = VertexArray::new();
        let Some(vao) = vao_null else {
            return Err("couldn't make a vao");
        };
        vao.bind();
        self.vao = vao;

        let vbo_null = Buffer::new();
        let Some(vbo) = vbo_null else {
            return Err("couldn't make a vbo");
        };
        vbo.bind(BufferType::Array);
        self.vbo = vbo;

        let ebo_null = Buffer::new();
        let Some(ebo) = ebo_null else {
            return Err("couldn't make a ebo");
        };
        ebo.bind(BufferType::ElementArray);
        self.ebo = ebo;

        let shader_program_ex =
            ShaderProgram::from_vert_frag_file(vert, frag).inspect_err(|e| println!("{}", e));
        let Ok(shader_program) = shader_program_ex else {
            return Err("couldn't make shader program");
        };
        self.shader_program = shader_program;
        Ok(())
    }

    /// Deletes the window.
    ///
    /// Comsumes `self`.
    pub fn delete(self) {
        unsafe {
            glDeleteVertexArrays(1, self.vao.0 as *const _);
            glDeleteBuffers(1, self.vbo.0 as *const _);
            glDeleteBuffers(1, self.ebo.0 as *const _);
        }
    }

    /// Executes the render loop
    /// # Note
    /// The loop doesn't run in a different thread
    pub fn render_loop(&self, meshes: &Vec<Mesh>) {
        'main_loop: loop {
            while let Some(event) = self.sdl.poll_events() {
                if let (Event::Quit, _) = event {
                    break 'main_loop;
                }
            }

            unsafe {
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                self.shader_program
                    .set_matrix4("transform\0", Mat4::identity());

                for msh in meshes {
                    buffer_data(
                        BufferType::Array,
                        bytemuck::cast_slice(msh.to_vertex_data_internal().as_slice()),
                        GL_DYNAMIC_DRAW,
                    );
                    buffer_data(
                        BufferType::ElementArray,
                        bytemuck::cast_slice(msh.indices.as_slice()),
                        GL_DYNAMIC_DRAW,
                    );

                    glDrawElements(
                        GL_TRIANGLES,
                        msh.indices.len() as i32,
                        GL_UNSIGNED_INT,
                        ptr::null(),
                    );
                    self.shader_program.use_program();
                }
            }

            self.window.swap_window();
        }
    }

    /// Creates the Sdl with approprate flags set
    /// # Returns
    /// - Sdl
    fn init_sdl() -> Sdl {
        let sdl = Sdl::init(InitFlags::EVERYTHING);
        sdl.set_gl_context_major_version(3).unwrap();
        sdl.set_gl_context_major_version(3).unwrap();
        sdl.set_gl_profile(GlProfile::Core).unwrap();

        let mut flags = GlContextFlags::default();

        if cfg!(target_os = "macos") {
            flags |= GlContextFlags::FORWARD_COMPATIBLE;
        }
        sdl.set_gl_context_flags(flags).unwrap();
        sdl
    }
}

impl Default for Window {
    /// Creates a window with the default `CreateWinArgs`
    /// # Returns
    /// default window
    /// # Panics
    /// - When the window can't be created. To avoid this use the `::new` method.
    fn default() -> Self {
        let win_args = CreateWinArgs {
            title: "window",
            width: 800,
            height: 600,
            allow_high_dpi: true,
            borderless: false,
            resizable: false,
        };

        let win_ex = Self::new(win_args);
        match win_ex {
            Ok(win) => win,
            Err(err) => panic!("{}", err),
        }
    }
}

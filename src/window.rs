//! Used for the `Window` helper structure. Containing various GL objects.

use std::ptr;

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlWindow},
    *,
};
use ogl33::*;
use uuid::Uuid;

use crate::{
    entities::{entity::EntityType, entity_tree::EntityTree},
    gl_helper::*,
};

/// Takes a string literal and concatenates a null byte onto the end.
#[macro_export]
macro_rules! null_str {
    ($lit:literal) => {{
        const _: &str = $lit;
        concat!($lit, "\0")
    }};
}
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
            ShaderProgram::from_vert_frag(vert, frag).inspect_err(|e| println!("{}", e));
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

    fn render_part(&self, entity_tree: &EntityTree, part_id: &Uuid) {
        let entity_null = entity_tree.get_entity(*part_id);
        let Some(entity) = entity_null else {
            return;
        };
        let EntityType::Part(part) = entity.get_type() else {
            return;
        };

        if !part.visable {
            return;
        }

        let transform = part.transform;
        self.shader_program
            .set_matrix4(null_str!("model"), transform);
        self.shader_program
            .set_color3(null_str!("obj_color"), part.color);

        let mesh = part.get_mesh();

        buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(mesh.to_vertex_data_internal().as_slice()),
            GL_DYNAMIC_DRAW,
        );
        buffer_data(
            BufferType::ElementArray,
            bytemuck::cast_slice(mesh.indices.as_slice()),
            GL_DYNAMIC_DRAW,
        );

        let texture_null = part.get_texture();

        if let Some(texture) = texture_null {
            unsafe {
                glBindTexture(GL_TEXTURE_2D, texture.texture_id);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
                glTexImage2D(
                    GL_TEXTURE_2D,
                    0,
                    GL_RGBA as GLint,
                    texture.width as GLsizei,
                    texture.height as GLsizei,
                    0,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    texture.pixels.cast(),
                );
                glGenerateMipmap(GL_TEXTURE_2D);

                glDrawElements(
                    GL_TRIANGLES,
                    mesh.indices.len() as i32,
                    GL_UNSIGNED_INT,
                    ptr::null(),
                );
                self.shader_program.use_program();
            }
        }
    }

    /// Executes the render loop
    /// # Note
    /// The loop doesn't run in a different thread
    pub fn render_loop(&self, entity_tree: &EntityTree) {
        let head_binding = entity_tree.get_head().unwrap();
        let head = head_binding.borrow();
        let input_service_entity_null = entity_tree.find_first_child_mut(&head, "InputService");
        let Some(mut input_service_entity) = input_service_entity_null else {
            panic!("couldn't find service Entity InputService");
        };

        'main_loop: loop {
            let EntityType::InputService(input_service) = input_service_entity.get_type_mut()
            else {
                panic!("couldn't borrow InputService");
            };

            while let Some((event, _timestamp)) = self.sdl.poll_events() {
                match event {
                    Event::Quit => break 'main_loop,
                    Event::Key {
                        pressed, keycode, ..
                    } => {
                        input_service.provide_input(keycode, pressed);
                    }
                    _ => (),
                }
            }

            unsafe {
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            }

            let main_camera_null = entity_tree.get_main_camera();

            if let Some(main_camera) = main_camera_null {
                let main_camera_borrow = main_camera.borrow();

                let EntityType::Camera(camera) = main_camera_borrow.get_type() else {
                    panic!("camera doesn't isn't a camera type");
                };

                let window_size = self.window.get_window_size();
                let aspect_ratio = (window_size.0 as f32) / (window_size.1 as f32);

                let view = camera.transform; // Mat4::from_translation(Vec3::new(0.0, 0.0, -1.0))
                let projection = camera.get_projection(aspect_ratio);

                self.shader_program
                    .set_matrix4(null_str!("projection"), projection);
                self.shader_program.set_matrix4(null_str!("view"), view);

                for part_id in &entity_tree.parts {
                    self.render_part(entity_tree, part_id);
                }
            }

            self.window.swap_window();

            let EntityType::InputService(input_service) = input_service_entity.get_type_mut()
            else {
                panic!("couldn't borrow InputService");
            };
            input_service.mark_cleanup();
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

#![cfg_attr(not(debug_assertions), window_subsystem = "windows")]
pub mod gl_helper;

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval},
    *,
};
use core::{convert::TryInto, mem::size_of};
use ogl33::*;

use crate::gl_helper::*;

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
const WINDOW_TITLE: &str = "Test Window";

const VERT_SHADER: &str = r#"#version 330 core
    layout (location = 0) in vec3 pos;
    void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
    out vec4 final_color;

    void main() {
        final_color = vec4(1.0, 0.5, 0.2, 1.0);
}
"#;

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

    return sdl;
}

fn main() {
    let sdl = init_sdl();

    let win_args = CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
        ..Default::default()
    };

    let win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");
    let _ = win.set_swap_interval(GlSwapInterval::Vsync).unwrap();

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
    }

    clear_color(0.2, 0.3, 0.3, 1.0);

    let vao = VertexArray::new().expect("couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new().expect("couldn't make a VBO");
    vbo.bind(BufferType::Array);
    gl_helper::buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        GL_STATIC_DRAW,
    );

    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);

        let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
        shader_program.use_program();
    }

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events() {
            match event {
                (Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
        win.swap_window();
    }
}

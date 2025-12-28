#![cfg_attr(not(debug_assertions), window_subsystem = "windows")]
pub mod gl_helper;
pub mod window;

use beryllium::video::{CreateWinArgs, GlSwapInterval};
use core::{convert::TryInto, mem::size_of};
use ogl33::*;

use crate::gl_helper::*;
use crate::window::*;

type VertexData = [f32; 6];
type TriIndexes = [u32; 3];

const VERTICES: [VertexData; 4] = [
    [0.5, 0.5, 0.0, 1.0, 0.0, 0.0],
    [0.5, -0.5, 0.0, 0.0, 1.0, 0.0],
    [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0],
    [-0.5, 0.5, 0.0, 1.0, 1.0, 1.0],
];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];
const WINDOW_TITLE: &str = "Test Window";

const VERT_SHADER: &str = "src/shaders/vert.glsl";
const FRAG_SHADER: &str = "src/shaders/frag.glsl";

fn main() {
    let win_args = CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
        ..Default::default()
    };

    let mut win = Window::new(win_args).unwrap();
    let gl_window = &win.window;

    let _ = gl_window.set_swap_interval(GlSwapInterval::Vsync).unwrap();

    unsafe {
        load_gl_with(|f_name| gl_window.get_proc_address(f_name.cast()));
    }

    clear_color(0.2, 0.3, 0.3, 1.0);

    win.init_objects().unwrap();

    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        GL_STATIC_DRAW,
    );

    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        GL_STATIC_DRAW,
    );

    unsafe {
        let vertex_data_size = size_of::<VertexData>().try_into().unwrap();

        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, vertex_data_size, 0 as *const _);
        glEnableVertexAttribArray(0);

        glVertexAttribPointer(
            1,
            3,
            GL_FLOAT,
            GL_FALSE,
            vertex_data_size,
            (3 * size_of::<f32>()) as *const _,
        );
        glEnableVertexAttribArray(1);
    }

    let shader_program = ShaderProgram::from_vert_frag_file(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    polygon_mode(gl_helper::PolygonMode::Fill);
    win.render_loop();
}

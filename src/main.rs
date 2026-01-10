//! This the entry point of *akhiok-engine game client*
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod gl_helper;
pub mod mesh;
pub mod texture;
/// Contains common datatypes used inside the engine.
pub mod datatypes {
    pub mod color;
    pub mod vectors;
}
/// Contains types used in the entity heirarchry structure.
pub mod entities {
    pub mod entity;
    pub mod entity_tree;
    /// Contains all variants of entities
    pub mod types {
        pub mod camera_type;
        pub mod game_type;
        pub mod part_type;
    }
    /// Contains common entity traits
    pub mod traits {
        pub mod object_3d;
        pub mod update;
    }
}
pub mod window;

use beryllium::video::{CreateWinArgs, GlSwapInterval};
use core::{convert::TryInto, mem::size_of};
use ogl33::*;
use std::ptr;

use crate::{
    datatypes::{color::Color3, vectors::Vector3},
    entities::{
        entity::EntityType,
        entity_tree::EntityTree,
        traits::object_3d::Object3D,
        types::{camera_type::CameraType, part_type::PartType},
    },
    gl_helper::*,
    mesh::*,
    texture::*,
    window::*,
};

/// The default window title
const WINDOW_TITLE: &str = "Test Window";

/// The path of the vertex shader
const VERT_SHADER: &str = "src/shaders/vert.glsl";
/// The path of the fragmentation shader
const FRAG_SHADER: &str = "src/shaders/frag.glsl";

fn start_window() -> Window {
    let win_args = CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let mut win = Window::new(win_args).unwrap();
    let gl_window = &win.window;
    gl_window.set_swap_interval(GlSwapInterval::Vsync).unwrap();
    unsafe {
        load_gl_with(|f_name| gl_window.get_proc_address(f_name.cast()));
    }

    clear_color(Color3::new(0.2, 0.3, 0.3).unwrap());
    win.init_objects(VERT_SHADER, FRAG_SHADER).unwrap();
    win
}

fn init_test_tree(entity_tree: &mut EntityTree) {
    let mesh = Mesh::load_mesh_from_file("assets/meshs/plane.mesh").unwrap();

    let head = entity_tree.add_head();
    println!("{}", head.borrow().get_uuid());

    let mut part_type = Box::new(PartType::new(&mesh));
    part_type.color = Color3::from_hex(0xff0000);

    let bitmap = Texture::from_file("assets/awesomeface.png").unwrap();
    part_type.set_texture(bitmap);

    let mut head_borrow = head.borrow_mut();

    let mut camera_type = CameraType::new(90.0, 0.1, 100.0);
    camera_type.set_rotation(Vector3::new(0.0, 10.0, 0.0));
    camera_type.set_position(Vector3::forward() * -1.0);

    let _ = entity_tree.add_main_camera(Some(&mut head_borrow), camera_type);
    let _ = entity_tree
        .add_entity_with_parent("part-entity", EntityType::Part(part_type), &mut head_borrow)
        .unwrap();
}

fn enable_vertex_arrays() {
    unsafe {
        let vertex_data_size = size_of::<VertexDataInternal>().try_into().unwrap();

        // position
        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, vertex_data_size, ptr::null());
        glEnableVertexAttribArray(0);

        // texture
        glVertexAttribPointer(
            1,
            2,
            GL_FLOAT,
            GL_FALSE,
            vertex_data_size,
            size_of::<[f32; 3]>() as *const _,
        );
        glEnableVertexAttribArray(1);
    }
}

/// main function
fn main() {
    let win = start_window();
    let mut entity_tree = EntityTree::default();
    init_test_tree(&mut entity_tree);

    win.shader_program.use_program();

    enable_vertex_arrays();

    polygon_mode(gl_helper::PolygonMode::Fill);
    win.render_loop(&entity_tree);
    win.shader_program.delete();
}

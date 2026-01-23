//! This the entry point of *akhiok-engine game client*
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(missing_docs)]
#![deny(clippy::all)]
#![allow(mismatched_lifetime_syntaxes)]

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
        pub mod io_service;
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
use std::{cell::RefCell, ptr, rc::Rc};

use crate::{
    datatypes::{color::Color3, vectors::Vector3},
    entities::{
        entity::{Entity, EntityType},
        entity_tree::EntityTree,
        traits::object_3d::Object3D,
        types::{
            camera_type::Camera,
            game_type::{Game, GameGenre},
            io_service::InputService,
            part_type::Part,
        },
    },
    gl_helper::*,
    mesh::*,
    texture::*,
    window::*,
};

/// The default window title
const WINDOW_TITLE: &str = "Test Window";

/// The contents of the vertex shader file
const VERT_SHADER: &str = include_str!("shaders/vert.glsl");
/// The contents of the fragmentation shader file
const FRAG_SHADER: &str = include_str!("shaders/frag.glsl");

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

fn create_tree() -> (Rc<RefCell<EntityTree>>, Rc<RefCell<Entity>>) {
    let entity_tree = EntityTree::default();
    let tree_cell = Rc::new(RefCell::new(entity_tree));

    let tree_binding = tree_cell.clone();
    let mut tree_borrow = tree_binding.borrow_mut();

    let game_type = Game::new(GameGenre::Adventure);
    let head = tree_borrow.add_head(game_type);

    (tree_cell, head)
}

fn init_test_tree(entity_tree: Rc<RefCell<EntityTree>>, head: Rc<RefCell<Entity>>) {
    let mesh = Mesh::load_mesh(include_str!("../assets/meshs/plane.mesh")).unwrap();
    let bitmap = Texture::new(include_bytes!("../assets/awesomeface.png").to_vec());

    let mut tree = entity_tree.borrow_mut();

    let mut part_type = Part::new(&mesh);
    part_type.set_texture(bitmap);
    part_type.color = Color3::from_hex(0xff0000);

    drop(head);

    let mut camera_type = Camera::new(90.0, 0.1, 100.0);
    camera_type.set_rotation(Vector3::new(0.0, 10.0, 0.0));
    camera_type.set_position(Vector3::forward() * -1.0);

    let _ = tree.add_main_camera(camera_type).unwrap();

    let head = tree.get_head().unwrap();
    let mut head_borrow = head.borrow_mut();

    let _ = tree
        .add_entity_with_parent("part-entity", EntityType::Part(part_type), &mut head_borrow)
        .unwrap();

    let _ = tree.add_entity_with_parent(
        "InputService",
        EntityType::InputService(InputService::default()),
        &mut head_borrow,
    );
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
    let (tree_cell, head) = create_tree();

    let win = start_window();
    init_test_tree(tree_cell.clone(), head);

    win.shader_program.use_program();

    enable_vertex_arrays();

    polygon_mode(gl_helper::PolygonMode::Fill);
    win.render_loop(tree_cell);
    win.shader_program.delete();
}

// Test Section

#[test]
fn test_to_hsv_color_pure() {
    // pure colors
    let pure_white = Color3::from_hsv(0, 0.0, 1.0).unwrap();
    let pure_black = Color3::from_hsv(0, 0.0, 0.0).unwrap();

    let pure_red = Color3::from_hsv(0, 1.0, 1.0).unwrap();
    let pure_green = Color3::from_hsv(120, 1.0, 1.0).unwrap();
    let pure_blue = Color3::from_hsv(240, 1.0, 1.0).unwrap();

    assert_eq!(pure_white, Color3::white());
    assert_eq!(pure_black, Color3::black());

    assert_eq!(pure_red, Color3::red());
    assert_eq!(pure_green, Color3::green());
    assert_eq!(pure_blue, Color3::blue());
}

#[test]
fn test_entity_head() {
    let (_, head) = create_tree();

    assert_eq!(head.borrow().parent_id, None);
}

#[test]
fn test_add_entity() {
    let (tree_cell, head_binding) = create_tree();

    let mut head = head_binding.borrow_mut();
    let mut tree = tree_cell.borrow_mut();

    let test_entity_binding = tree
        .add_entity_with_parent(
            "test entity",
            EntityType::Base(entities::entity::Base),
            &mut head,
        )
        .unwrap();
    let test_entity = test_entity_binding.borrow_mut();

    assert_eq!(head.children_id[0], test_entity.get_uuid());
    assert_eq!(head.get_uuid(), test_entity.parent_id.unwrap());
}

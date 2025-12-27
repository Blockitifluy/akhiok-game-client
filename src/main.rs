#![cfg_attr(not(debug_assertions), window_subsystem = "windows")]
use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile},
    *,
};

const WINDOW_TITLE: &str = "Test Window";

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

    let _win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events() {
            match event {
                (Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
    }
}

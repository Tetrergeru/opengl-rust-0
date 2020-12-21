extern crate cgmath;
extern crate gl;
extern crate sdl2;

mod drawing;
mod entities;
mod world;

use drawing::{Cube, Program, Shader};
use world::World;

use cgmath::Rad;

fn main() {
    let w = 700.0f32;
    let h = 700.0f32;
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", w as u32, h as u32)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    sdl.mouse().show_cursor(false);
    sdl.mouse().set_relative_mouse_mode(true);

    let _gl_context = window.gl_create_context().unwrap();

    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    use std::ffi::CString;

    let vert_shader = Shader::from_vert_source(
        gl.clone(),
        &CString::new(include_str!("resources/triangle.vert")).unwrap(),
    )
    .unwrap();

    let frag_shader = Shader::from_frag_source(
        gl.clone(),
        &CString::new(include_str!("resources/triangle.frag")).unwrap(),
    )
    .unwrap();

    let shader_program = Program::new(gl.clone(), &[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    let world = World::new(gl.clone());

    unsafe {
        gl.Viewport(0, 0, w as gl::types::GLint, h as gl::types::GLint);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
        gl.Enable(gl::DEPTH_TEST);
    }

    let mut camera = drawing::Camera::new((0.0, 0.0, 0.0).into(), Rad(0.0), Rad(0.0), w / h);

    let mut keys = std::collections::HashSet::new();
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode, .. } => match keycode {
                    None => {}
                    Some(code) => {
                        keys.insert(code);
                    }
                },
                sdl2::event::Event::KeyUp { keycode, .. } => match keycode {
                    None => {}
                    Some(code) => {
                        keys.remove(&code);
                    }
                },
                sdl2::event::Event::MouseMotion { xrel, yrel, .. } => {
                    camera.rotate_horisontal(Rad(-xrel as f32 * 2.0 / h));
                    camera.rotate_vertical(Rad(-yrel as f32 * 2.0 / w));
                }
                _ => {}
            }
        }

        const STEP: f32 = 0.05;

        for key in keys.iter() {
            match key {
                sdl2::keyboard::Keycode::W => {
                    camera.move_forward(STEP);
                }
                sdl2::keyboard::Keycode::S => {
                    camera.move_forward(-STEP);
                }
                sdl2::keyboard::Keycode::D => {
                    camera.move_right(STEP);
                }
                sdl2::keyboard::Keycode::A => {
                    camera.move_right(-STEP);
                }
                _ => {}
            }
        }
        world.draw(&camera, &shader_program);
        window.gl_swap_window();
    }
}

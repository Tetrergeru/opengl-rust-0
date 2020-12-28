extern crate cgmath;
extern crate gl;
extern crate image;
extern crate sdl2;

mod drawing;
mod entities;
mod world;

use drawing::{Program, Shader};
use world::World;

use cgmath::Rad;
use sdl2::keyboard::Keycode;
use std::ffi::CString;

fn main() {
    let w = 1000.0f32;
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

    // let vert_shader_phong = Shader::from_vert_source(
    //     gl.clone(),
    //     &CString::new(include_str!("resources/guro.vert")).unwrap(),
    // )
    // .unwrap();

    // let frag_shader = Shader::from_frag_source(
    //     gl.clone(),
    //     &CString::new(include_str!("resources/guro.frag")).unwrap(),
    // )
    // .unwrap();

    let programs = [load_program(gl.clone(), "guro"), load_program(gl.clone(), "phong")];
    let mut current_program = 0;
    //Program::new(gl.clone(), &[vert_shader_phong, frag_shader]).unwrap();
    //shader_program.set_used();

    let mut world = World::new(gl.clone());

    unsafe {
        gl.Viewport(0, 0, w as gl::types::GLint, h as gl::types::GLint);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
        gl.Enable(gl::DEPTH_TEST);
    }

    let mut camera = drawing::Camera::new((0.0, 0.0, 0.0).into(), Rad(0.0), Rad(0.0), w / h);

    let mut keys = std::collections::HashSet::new();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut time = std::time::Instant::now();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode, .. } => match keycode {
                    None => {}
                    Some(code) => match code {
                        Keycode::F1 => {
                            world.color_coeff += 0.1;
                            if world.color_coeff > 1.0 {
                                world.color_coeff = 1.0;
                            }
                        }
                        Keycode::F2 => {
                            world.color_coeff -= 0.1;
                            if world.color_coeff < 0.0 {
                                world.color_coeff = 0.0;
                            }
                        }
                        Keycode::F3 => {
                            world.texture_coeff += 0.1;
                            if world.texture_coeff > 1.0 {
                                world.texture_coeff = 1.0;
                            }
                        }
                        Keycode::F4 => {
                            world.texture_coeff -= 0.1;
                            if world.texture_coeff < 0.0 {
                                world.texture_coeff = 0.0;
                            }
                        }
                        Keycode::F5 => {
                            world.turn_sun();
                        }
                        Keycode::F6 => {
                            world.turn_projector();
                        }
                        Keycode::F7 => {
                            current_program = match current_program {
                                0 => 1,
                                1 => 0,
                                _ => 0,
                            };
                        }
                        _ => {
                            keys.insert(code);
                        }
                    },
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

        const STEP: f32 = 15.0;
        let new_time = std::time::Instant::now();
        let delta_time = (new_time - time).as_secs_f32();
        let step = delta_time * STEP;
        time = new_time;
        for key in keys.iter() {
            match key {
                Keycode::W => {
                    camera.move_forward(step);
                }
                Keycode::S => {
                    camera.move_forward(-step);
                }
                Keycode::D => {
                    camera.move_right(step);
                }
                Keycode::A => {
                    camera.move_right(-step);
                }
                Keycode::Space => {
                    camera.move_vec((0.0, step, 0.0).into());
                }
                Keycode::LShift | Keycode::RShift => {
                    camera.move_vec((0.0, -step, 0.0).into());
                }
                _ => {}
            }
        }
        world.tick(&camera, &programs[current_program], delta_time);
        window.gl_swap_window();
    }
}

fn load_program(gl: gl::Gl, name: &str) -> Program {
    let vert_shader_phong = Shader::from_vert_source(
        gl.clone(),
        &CString::new(std::fs::read_to_string(format!("src/resources/{}.vert", name)).unwrap()).unwrap(),
    )
    .unwrap();

    let frag_shader = Shader::from_frag_source(
        gl.clone(),
        &CString::new(std::fs::read_to_string(format!("src/resources/{}.frag", name)).unwrap()).unwrap(),
    )
    .unwrap();

    Program::new(gl.clone(), &[vert_shader_phong, frag_shader]).unwrap()
}
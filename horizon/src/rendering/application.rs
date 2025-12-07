use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::event::*;
use glutin::event_loop::*;
use glutin::window::*;

use std::time::{Instant};

use crate::rendering::camera::Camera;
use crate::rendering::renderer::Renderer;

pub struct  WindowHandle {
    pub event_loop: EventLoop<()>,
    pub context: ContextWrapper<PossiblyCurrent, Window>,
}

#[allow(unsafe_op_in_unsafe_fn, unused)]
pub unsafe fn window_init(title: &str) -> WindowHandle {
    let event_loop: EventLoop<()> = EventLoop::new();
    let window: WindowBuilder = WindowBuilder::new()
        .with_title(title);

    let gl_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed((window), &event_loop)
        .expect("Error creating window");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Error making gl context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::Enable(gl::BLEND);
    gl::Enable(gl::CULL_FACE);

    WindowHandle {  
        event_loop: event_loop,
        context: gl_context,
    }
}

#[allow(unsafe_op_in_unsafe_fn, unused)]
pub unsafe fn window_event_loop(handle: WindowHandle, render_target: Renderer, camera: &Camera) {
    let mut last_frame_time = Instant::now();

    let mut camera: Camera = camera.clone();
    let mut camera_dx: f32 = 0.0;
    let mut camera_dy: f32 = 0.0;
    const CAMERA_ACC: f32 = 500.0;

    handle.event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        handle.context.window().request_redraw();

        /* calculate delta time */
        let current_time = Instant::now();
        let delta_time = current_time
            .duration_since(last_frame_time)
            .as_secs_f32();
        last_frame_time = current_time;

        match &mut camera {
            Camera::Orthographic(ortho) => {
                ortho.change_camera_position(camera_dx * delta_time, camera_dy * delta_time);
            },
            _ => ()
        }

        camera_dx *= (0.98);
        camera_dy *= (0.98);

        match event {
            Event::LoopDestroyed => (),

            ///////////////////
            // WINDOW EVENTS //
            ///////////////////
            Event::WindowEvent { event, .. } => {
                match event {

                    /* window close ----- */
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    /* window resize ----- */
                    WindowEvent::Resized(physical_size) => {
                        handle.context.resize(physical_size);

                        unsafe {
                            gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                        }

                        match camera {
                            Camera::Orthographic(mut ortho) => {
                                ortho.update_view_matrix(
                                    physical_size.width as f32, 
                                    physical_size.height as f32,
                                );
                            },
                            _ => ()
                        }

                    },

                    /* Up Arrow Event */
                    WindowEvent::KeyboardInput {
                        input: 
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Up), 
                                state: glutin::event::ElementState::Pressed,
                                ..
                            }, 
                        ..
                    } => {
                        camera_dy += CAMERA_ACC;
                    },

                    /* Down Arrow Event */
                    WindowEvent::KeyboardInput { 
                        input: 
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Down),
                                state: glutin::event::ElementState::Pressed,
                                ..
                            },
                        .. 
                    } => {
                        camera_dy -= CAMERA_ACC;
                    },

                    /* Left Arrow Event */
                    WindowEvent::KeyboardInput {
                        input: 
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Left), 
                                state: glutin::event::ElementState::Pressed,
                                ..
                            }, 
                        ..
                    } => {
                        camera_dx -= CAMERA_ACC;
                    },

                    /* Right Arrow Event */
                    WindowEvent::KeyboardInput { 
                        input: 
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Right),
                                state: glutin::event::ElementState::Pressed,
                                ..
                            },
                        .. 
                    } => {
                        camera_dx += CAMERA_ACC;
                    },

                    _ => ()
                }
            },

            ////////////////
            // DRAW CALLS //
            ////////////////
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0,);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    render_target.draw_requests(&camera);

                    handle.context.swap_buffers().unwrap();
                }
            }

            // misc events
            _ => (),
        }
    });

}
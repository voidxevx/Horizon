use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::dpi::PhysicalSize;
use glutin::event::*;
use glutin::event_loop::*;
use glutin::window::*;

use crate::rendering::camera::Camera;
use crate::rendering::renderer::Renderer;
use crate::tools::math;

pub struct WindowProps {
    title: String,
    base_color: [f32; 4]
}

impl WindowProps {
    pub fn new(title: String, base_color: [f32; 4]) -> Self {
        Self {
            title: title,
            base_color: base_color
        }
    }
}

#[allow(unused)]
pub struct App {
    event_loop: EventLoop<()>,
    context: ContextWrapper<PossiblyCurrent, Window>,
    window_props: WindowProps,
    camera: Camera,
    pub renderer: Renderer,
}

#[allow(unused)]
impl App {
    pub fn create(props: WindowProps) -> Self {
        let event_loop = EventLoop::new();
        let window: WindowBuilder = WindowBuilder::new()
            .with_title(props.title.clone());

        let gl_context = ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
            .build_windowed(window, &event_loop)
            .expect("Error creating window context");

        let gl_context = unsafe {
            gl_context
                .make_current()
                .expect("unable to make context current")
        };

        gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
        }

        Self  {
            event_loop: event_loop,
            context: gl_context,
            window_props: props,
            camera: Camera::None,
            renderer: Renderer::new(),
        }
    }

    pub fn attach_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn main_loop(self) {
        self.event_loop.run( move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
        
            let mut camera = self.camera;

            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent {event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        self.context.resize(physical_size);

                        match camera {
                            Camera::Orthographic(mut ortho) => {
                                println!("{:?}", ortho);
                                ortho.update_view_matrix(
                                    physical_size.width as f32,
                                    physical_size.height as f32,
                                );
                            },
                            _ => (),
                        }

                        unsafe {
                            gl::Viewport(
                                0,
                                0,
                                physical_size.width as i32,
                                physical_size.height as i32,
                            );
                        }
                    },
                    _ => (),
                },

                Event::RedrawRequested(_) => {
                    unsafe {
                        gl::ClearColor(
                            self.window_props.base_color[0].clone(), 
                            self.window_props.base_color[1].clone(), 
                            self.window_props.base_color[2].clone(), 
                            self.window_props.base_color[3].clone()
                        );
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }

                    self.renderer.draw_requests(camera);

                    self.context.swap_buffers().unwrap();
                },

                _ => (),
            }
        });
    }
}

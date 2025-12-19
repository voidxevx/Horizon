use gl::types::GLint;
use gl::types::GLsizei;
use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::event::*;
use glutin::event_loop::*;
use glutin::window::*;

use std::sync::Arc;
use std::time::{Instant};

use crate::rendering::material::Material;
use crate::rendering::material::MaterialInstance;
use crate::rendering::material::instance_material;
use crate::rendering::mesh_data::shader::ShaderUniform;
use crate::rendering::mesh_data::shader::ShaderUniformType;
use crate::rendering::mesh_data::shader_types::DataTypes;
use crate::rendering::mesh_data::vertex_layout::VertexLayout;
use crate::rendering::render_target;
use crate::rendering::render_target::MeshBuilder;
use crate::set_attribute;
use crate::tools::math::transforms::translation_matrix;
use crate::tools::math::vector::Vector;
use crate::tools::math::vector::{Vec3, Vec2};
use crate::{
    rendering::{
        render_target::{RenderTarget},
        mesh_data::{
            buffer::Buffer, 
            vertex_array::VertexArray,
        }, 
    },
};



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
pub unsafe fn window_event_loop(handle: WindowHandle, target_type: i32) {
    let mut last_frame_time = Instant::now();

    let test_material: Arc<Material> = Material::new("./content/materials/default.mat")
        .expect("Error loading material");


    let mesh: [f32; 20] = [
        0.0,   0.0,   0.0,    0.0, 1.0,
        100.0, 0.0,   0.0,    1.0, 1.0,
        100.0, 100.0, 0.0,    1.0, 0.0,
        0.0,   100.0, 0.0,    0.0, 0.0,
    ];


    let indeces: [i32; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];

    
    
    let vertex_array = VertexArray::new();
    vertex_array.bind();
    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.buffer_data(&mesh, gl::STATIC_DRAW);
    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    index_buffer.buffer_data(&indeces, gl::STATIC_DRAW);
    

    let shader = &test_material.shader_program;
    
    vertex_array.bind();
    let layout = VertexLayout::new()
        .attrib(DataTypes::Float3)
        .attrib(DataTypes::Float2);
    layout.bind_attribute();

    let mut render_target = RenderTarget::new(target_type);

    MeshBuilder::new(vertex_array.clone(), test_material.clone())
        .uniform("projectionMatrix", 
            ShaderUniform::MatrixUniform(
                translation_matrix(Vector::Length3(Vec3::new([100.0, 0.0, 0.0]))).unwrap()
            )
        )
    .attach(&mut render_target);

    MeshBuilder::new(vertex_array.clone(), test_material.clone())
        .uniform("projectionMatrix", 
            ShaderUniform::MatrixUniform(
                translation_matrix(Vector::Length3(Vec3::new([300.0, 0.0, 0.0]))).unwrap()
            )
        )
    .attach(&mut render_target);

    MeshBuilder::new(vertex_array.clone(), test_material.clone())
        .uniform("projectionMatrix", 
            ShaderUniform::MatrixUniform(
                translation_matrix(Vector::Length3((Vec3::new([300.0, 450.0, 0.0])))).unwrap()
            )
        )
    .attach(&mut render_target);

    MeshBuilder::new(vertex_array.clone(), test_material.clone())
        .uniform("projectionMatrix", 
            ShaderUniform::MatrixUniform(
                translation_matrix(Vector::Length3((Vec3::new([100.0, 450.0, 0.0])))).unwrap()
            )
        )
    .attach(&mut render_target);



    //////////////////////
    // EVENT LOOP START //
    /////////////////////
    handle.event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        handle.context.window().request_redraw();

        /* calculate delta time */
        let current_time = Instant::now();
        let delta_time = current_time
            .duration_since(last_frame_time)
            .as_secs_f32();
        last_frame_time = current_time;

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
                        unsafe { gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32); }
                        render_target.resize_capture(physical_size.width as f32, physical_size.height as f32);
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
                    render_target.capture();
                    // debug_manager.draw_widgets();
                    handle.context.swap_buffers().unwrap();
                }
            }

            // misc events
            _ => (),
        }
    });

}
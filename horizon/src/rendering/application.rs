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
use crate::set_attribute;
use crate::tools::debug_widgets::widget::DebugGuiManager;
use crate::tools::debug_widgets::widget::DebugWidget;
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

#[repr(C, packed)]
struct Vertex (Vec3, Vec2);

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
pub unsafe fn window_event_loop(handle: WindowHandle, target_types: Vec<i32>) {
    let mut last_frame_time = Instant::now();

    let mesh = [
        // coords              // texcoords
        Vertex( Vec3::new([0.0,   0.0,   0.0]), Vec2::new([0.0, 1.0]) ),
        Vertex( Vec3::new([100.0, 0.0,   0.0]), Vec2::new([1.0, 1.0]) ),
        Vertex( Vec3::new([100.0, 100.0, 0.0]), Vec2::new([1.0, 0.0]) ),
        Vertex( Vec3::new([0.0,   100.0, 0.0]), Vec2::new([0.0, 0.0]) ),
    ];


    let indeces: [i32; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];

    
    let test_material: Arc<Material> = Material::new("./content/materials/default.mat")
        .expect("Error loading material");

    let test_instance: MaterialInstance = instance_material(test_material.clone());

    let vertex_array = VertexArray::new();
    vertex_array.bind();
    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.buffer_data(&mesh, gl::STATIC_DRAW);
    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    index_buffer.buffer_data(&indeces, gl::STATIC_DRAW);

    let shader = &test_material.shader_program;

    let loc_attrib = shader.get_attrib_location("loc").expect("attribute not found");
    set_attribute!(vertex_array, loc_attrib, Vertex::0);
    let tex_attrib = shader.get_attrib_location("vertTexCoords").expect("attribute not found");
    set_attribute!(vertex_array, tex_attrib, Vertex::1);



    let mut render_targets: Vec<RenderTarget> = Vec::new();
    for target_type in target_types {
        render_targets.push(
            RenderTarget::new(target_type)
        );
    }

    let mut other = instance_material(test_material);
    other.set_uniform(&String::from("projectionMatrix"), ShaderUniform::MatrixUniform(translation_matrix(Vector::Length3(Vec3::new([100.0, 200.0, 0.0]))).unwrap()));

    render_targets[0].add_draw_request(vertex_array.clone(), test_instance);
    render_targets[0].add_draw_request(vertex_array.clone(), other);

    let mut debug_manager: DebugGuiManager = DebugGuiManager::new();
    
    let test_widget: DebugWidget = DebugWidget::new(200.0, 400.0)
        .located_at(100.0, 100.0);

    debug_manager.add_widget(test_widget);

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
                        for render_target in &mut render_targets {
                            render_target.resize_capture(physical_size.width as f32, physical_size.height as f32);
                        }
                        debug_manager.update_gui(physical_size.width as f32, physical_size.height as f32);

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
                    for render_target in &mut render_targets {
                        render_target.capture();
                    }
                    // debug_manager.draw_widgets();
                    handle.context.swap_buffers().unwrap();
                }
            }

            // misc events
            _ => (),
        }
    });

}
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
use crate::set_attribute;
use crate::{
    rendering::{
        render_target::{RenderTarget},
        mesh_data::{
            buffer::Buffer, 
            shader::*, 
            texture::Texture, 
            vertex_array::VertexArray,
        }, 
    },

    tools::math::vector::{
        Vec2, 
        Vec3
    }
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

#[allow(unused)]
#[repr(C, packed)]
struct Vertex(Vec3, Vec2);


#[allow(unsafe_op_in_unsafe_fn, unused)]
pub unsafe fn window_event_loop(handle: WindowHandle, target_types: Vec<i32>) {
    let mut last_frame_time = Instant::now();

    //////////
    // TEMP //
    //////////
    let mesh: [Vertex; 4] = [
        Vertex(Vec3::new([0.0,   0.0,   0.0]), Vec2::new([0.0, 1.0])),
        Vertex(Vec3::new([100.0, 0.0,   0.0]), Vec2::new([1.0, 1.0])),
        Vertex(Vec3::new([100.0, 100.0, 0.0]), Vec2::new([1.0, 0.0])),
        Vertex(Vec3::new([0.0,   100.0, 0.0]), Vec2::new([0.0, 0.0])),
    ];

    let indeces: [i32; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];

    

    let test_material: Arc<Material> = Material::new("./content/materials/default.mat")
        .expect("Error loading material");

    let test_instance: MaterialInstance = instance_material(test_material);



    let shader = generate_shader("./content/shaders/default.shader").unwrap();

    let vertex_array = VertexArray::new();
    vertex_array.bind();

    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.buffer_data(&mesh, gl::STATIC_DRAW);

    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    index_buffer.buffer_data(&indeces, gl::STATIC_DRAW);

    let loc_attrib = shader.get_attrib_location("loc").expect("attribute not found");
    set_attribute!(vertex_array, loc_attrib, Vertex::0);
    let tex_attrib = shader.get_attrib_location("vertTexCoords").expect("attribute not found");
    set_attribute!(vertex_array, tex_attrib, Vertex::1);

    let texture = Texture::new();
    texture.set_wrapping(gl::REPEAT);
    texture.load("./content/textures/tetosphere.png").expect("unable to load texture");
    shader.set_uniform("texture0", &ShaderUniform::IntUniform(0)).expect("unable to set uniform");




    let mut render_targets: Vec<RenderTarget> = Vec::new();
    for target_type in target_types {
        render_targets.push(
            RenderTarget::new(target_type)
        );
    }

    render_targets[0].add_draw_request(vertex_array, shader, texture);

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
                    for render_target in &render_targets {
                        render_target.capture();
                    }
                    handle.context.swap_buffers().unwrap();
                }
            }

            // misc events
            _ => (),
        }
    });

}
use glutin::{
    ContextBuilder,
    ContextWrapper,
    PossiblyCurrent,
    event::*,
    event_loop::*,
    window::*,
};

use std::{
    sync::Arc,
    time::Instant,
};

use crate::{
    rendering::{
        material::{
            Material,
            MaterialInstance,
            instance_material,
        },
        mesh_data::{
            shader::{
                ShaderUniform,
                ShaderUniformType,
            },
            shader_types::DataTypes,
            vertex_layout::VertexLayout,
            buffer::Buffer,
            vertex_array::VertexArray,
        },
        render_target::{
            RenderTarget,
            MeshBuilder,
        },
    },
    tools::{
        math::{
            transforms::{
                rotation_mat4_euler_z,
                scalar_matrix,
                translation_matrix,
            },
            vector::{
                Vector,
                Vec4,
                Vec3,
                Vec2
            }
        }
    },
    set_attribute
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

    let teto_material: Arc<Material> = Material::new("./content/materials/teto.mat")
        .expect("Error loading material");

    let miku_material: Arc<Material> = Material::new("./content/materials/miku.mat")
        .expect("Error loading material");



    let mesh: Vec<f32> = vec![
        -50.0, -50.0, 0.0,    0.0, 1.0,
         50.0, -50.0, 0.0,    1.0, 1.0,
         50.0,  50.0, 0.0,    1.0, 0.0,
        -50.0,  50.0, 0.0,    0.0, 0.0,
    ];


    let indecis = vec![
        0, 1, 2,
        2, 3, 0
    ];
    
    
    let vertex_array = VertexArray::new();
    vertex_array.bind();
    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.buffer_data(&mesh[..], gl::STATIC_DRAW);
    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    index_buffer.buffer_data(&indecis[..], gl::STATIC_DRAW);

    vertex_array.bind();
    let layout = VertexLayout::new()
        .attrib(DataTypes::Float3)
        .attrib(DataTypes::Float2);
    layout.bind_attribute();

    let mut render_target = RenderTarget::new(target_type);

    let teto = MeshBuilder::new(vertex_array.clone(), teto_material.clone())
        .uniform("projectionMatrix",
            ShaderUniform::MatrixUniform(
                translation_matrix(Vector::Length3(Vec3::new([450.0, 450.0, 0.0]))).unwrap()
            )
        )
        .uniform("scalarMatrix",
            ShaderUniform::MatrixUniform(
                scalar_matrix(Vector::Length4(Vec4::new([5.0, 2.5, 1.0, 1.0])))
            )
        )
        .uniform("rotationMatrix", 
            ShaderUniform::MatrixUniform(
                rotation_mat4_euler_z(35.0 * (3.14 / 180.0))
            )
        )
    .attach(&mut render_target);

    let miku = MeshBuilder::new(vertex_array.clone(), miku_material.clone())
        .uniform("projectionMatrix",
            ShaderUniform::MatrixUniform(
                translation_matrix(Vector::Length3(Vec3::new([750.0, 950.0, 0.0]))).unwrap()
            )
        )
        .uniform("scalarMatrix",
            ShaderUniform::MatrixUniform(
                scalar_matrix(Vector::Length4(Vec4::new([5.0, 2.5, 1.0, 1.0])))
            )
        )
        .uniform("rotationMatrix", 
            ShaderUniform::MatrixUniform(
                rotation_mat4_euler_z(-15.0 * (3.14 / 180.0))
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
                        gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32); 
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
                    handle.context.swap_buffers().unwrap();
                }
            }

            // misc events
            _ => (),
        }
    });

}
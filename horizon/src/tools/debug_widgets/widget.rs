use crate::{rendering::{mesh_data::{buffer::Buffer, shader::{ShaderProgram, generate_shader}, texture::Texture, vertex_array::VertexArray}, render_target::RenderTarget}, tools::math::vector::{Vec2, Vec3}};

#[allow(unused)]
#[repr(C, packed)]
struct WidgetVertex(Vec2, Vec2);


/*
 * Generates a reusable vertex array that will be used to generate debug widget sprites.
 */
#[allow(unused)]
pub fn generate_widget_sprite() -> VertexArray {
    let  widget_sprite_vertecis: [WidgetVertex; 4] = [
        WidgetVertex(Vec2::new([100.0, 100.0]), Vec2::new([1.0, 1.0])),
        WidgetVertex(Vec2::new([100.0,   0.0]), Vec2::new([1.0, 0.0])),
        WidgetVertex(Vec2::new([0.0,   100.0]), Vec2::new([0.0, 1.0])),
        WidgetVertex(Vec2::new([0.0,     0.0]), Vec2::new([0.0, 0.0])),
    ];
        
    let widget_sprite_indecis: [i32; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];

    let vertex_array: VertexArray = VertexArray::new();
    vertex_array.bind();

    let vertex_buffer: Buffer = Buffer::new(gl::ARRAY_BUFFER);
    let index_buffer: Buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

    vertex_buffer.buffer_data(&widget_sprite_vertecis, gl::STATIC_DRAW);
    index_buffer.buffer_data(&widget_sprite_indecis, gl::STATIC_DRAW);

    vertex_array
}

pub fn generate_widget_shader() -> ShaderProgram {
    generate_shader("./content/shaders/debug_flat_color_widget.shader")
        .expect("Error generating widget shader")
}

#[allow(unused)]
pub struct DebugWidget {
    background_color: Vec3,
    widget_location: Vec2,
    widget_scale: Vec2,
}

#[allow(unused)]
pub struct DebugWidgetManager {
    va: VertexArray,
    widgets: Vec<DebugWidget>,
    shaders: ShaderProgram,
}

#[allow(unused)]
impl DebugWidgetManager {
    pub fn new() -> Self {
        let va = generate_widget_sprite();
        let flat_shader = generate_widget_shader();

        Self {
            va: va,
            widgets: Vec::new(),
            shaders: flat_shader,
        }
    }

    pub fn add_widget(&mut self, color: Vec3, location: Vec2, scale: Vec2) {
        self.widgets.push(
            DebugWidget {
                background_color: color,
                widget_location: location,
                widget_scale: scale,
            }
        );
    }

    pub fn attach_widgets(&self, target: &mut RenderTarget) {
        let temp_texture: Texture = Texture { id: 0 };

    }
}
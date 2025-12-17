use std::sync::Arc;

use crate::{rendering::{material::{Material, MaterialInstance, instance_material}, mesh_data::{buffer::Buffer, shader::{ShaderProgram, ShaderUniform}, vertex_array::VertexArray}, render_target::{self, RENDER_TARGET_ORTHOGRAPHIC, RenderTarget}}, set_attribute, tools::math::{matrix::Matrix, transforms::scalar_matrix, vector::{Vec4, Vec2, Vec3, Vector}}};

#[repr(C, packed)]
struct WidgetSpriteVertex(Vec2);

struct WidgetHeader {
    enabled: bool,
    tint: Vec3,
    fill: f32,
}

pub struct DebugWidget {
    width: f32,
    height: f32,
    location: Vec2,
    color: Vec3,
    header: WidgetHeader,
}

impl DebugWidget {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width: width,
            height: height,
            location: Vec2::new([0.0, 0.0]),
            color: Vec3::new([0.36, 0.44, 0.63]),
            header: WidgetHeader {
                enabled: true, 
                tint: Vec3::new([0.3, 0.3, 0.3]),
                fill: 0.1,
            }
        }
    }

    pub fn with_header(mut self, enabled: bool) -> Self {
        self.header.enabled = enabled;
        self
    }

    pub fn with_color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.color = Vec3::new([r, g, b]);
        self
    }

    pub fn set_header(mut self, fill: f32, tint: f32) -> Self{
        self.header.fill = fill;
        self.header.tint = Vec3::new([tint, tint, tint]);
        self
    }

    pub fn located_at(mut self, x: f32, y: f32) -> Self {
        self.location = Vec2::new([x, y]);
        self
    }
}

pub struct DebugGuiManager {
    show: bool,
    va: Arc<VertexArray>,
    material: Arc<Material>,
    target: RenderTarget,
}

impl DebugGuiManager {
    pub fn new() -> Self {
        
        let widget_sprite = [
            WidgetSpriteVertex(Vec2::new([0.0, 0.0,])),
            WidgetSpriteVertex(Vec2::new([1.0, 0.0,])),
            WidgetSpriteVertex(Vec2::new([1.0, 1.0,])),
            WidgetSpriteVertex(Vec2::new([0.0, 1.0,])),
        ];

        let sprite_indecs: [i32; 6] = [
            0, 1, 2,
            2, 3, 0,
        ];

        let va = VertexArray::new();
        va.bind();
        let vb = Buffer::new(gl::ARRAY_BUFFER);
        vb.buffer_data(&widget_sprite, gl::STATIC_DRAW);
        let ib = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
        ib.buffer_data(&sprite_indecs, gl::STATIC_DRAW);

        let mat = Material::new("./content/materials/debugWidget.mat")
            .expect("Error loading debug material.");

        let location_attrib = mat.shader_program.get_attrib_location("location")
            .expect("location attribute for debug shader was not found.");
        set_attribute!(va, location_attrib, WidgetSpriteVertex::0);

        let target: RenderTarget = RenderTarget::new(RENDER_TARGET_ORTHOGRAPHIC);

        Self {
            show: true,
            va: va,
            material: mat,
            target: target,
        }
    }

    pub fn add_widget(&mut self, widget: DebugWidget) {
        let mut inst: MaterialInstance = instance_material(self.material.clone());
        inst.set_uniform(&String::from("widgetColor"), ShaderUniform::VectorUnform(Vector::Length3(widget.color.clone())));
        let scale_mat: Matrix = scalar_matrix(
            Vector::Length4(
                Vec4::new(
                    [widget.width, widget.height, 1.0, 1.0]
                )
            )
        );
        inst.set_uniform(&String::from("widgetScaleMatrix"), ShaderUniform::MatrixUniform(scale_mat));
        self.target.add_draw_request(self.va.clone(), inst);
    }

    pub fn update_gui(&mut self, width: f32, height: f32) {
        self.target.resize_capture(width, height);
    }

    pub fn draw_widgets(&mut self) {
        self.target.capture();
    }

}
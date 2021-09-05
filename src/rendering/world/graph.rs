use std::collections::HashMap;
use glium::Surface;
use legion::Entity;
use crate::math::Vector4;
use crate::mesh;
use crate::math;
use super::textures;
use super::shaders;
use glium::implement_vertex;

#[derive(Default)]
pub struct RenderGraphSet;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    colour: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, colour, tex_coords);

impl Into<Vec<Vertex>> for &mesh::Mesh {
    fn into(self) -> Vec<Vertex> {
        self.data
            .iter()
            .map(|mesh_vertex| mesh_vertex.into())
            .collect()
    }
}

impl Into<Vertex> for &mesh::Vertex {
    fn into(self) -> Vertex {
        Vertex {
            position: self.position.into(), 
            normal: self.normal.into(), 
            colour: self.colour.into(), 
            tex_coords: self.uv.into(), 
        }
    }
}

pub struct WorldRenderGraph {
    nodes: HashMap<Entity, WorldRenderGraphNode>,
    pub light_direction: math::Vector4,
    pub view: math::Matrix4x4
}

impl WorldRenderGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::default(),
            light_direction: math::Vector4::default(),
            view: math::Matrix4x4::identity()
        }
    }
    
    pub fn add_node(&mut self, entity: Entity, node: WorldRenderGraphNode) {
        self.nodes.insert(entity, node);
    }
    
    pub fn add_mesh(&mut self, entity: Entity, vertices: Vec<mesh::Vertex>) {
        self.add_node(entity, WorldRenderGraphNode::new(WorldRenderGraphNodeType::Mesh(MeshGraphNode::new(mesh::Mesh::new(vertices)))));
    }

    pub fn find(&mut self, entity: &Entity) -> Option<&mut WorldRenderGraphNode> {
        self.nodes.get_mut(entity)
    }

    pub fn draw(&mut self, target_dimensions: (u32, u32), display: &mut glium::Display, target: &mut glium::Frame) {
        let perspective = math::Matrix4x4::perspective(target_dimensions.0, target_dimensions.1);
        let view = self.view.clone();
        let light_direction = self.light_direction.clone();

        for node in self.nodes.values_mut() {
            node.draw(display, target, perspective, view, light_direction)
        }
    }
}

pub struct WorldRenderGraphNode {
    position: math::Matrix4x4,
    rotation: math::Matrix4x4,
    node_type: WorldRenderGraphNodeType,
}

impl WorldRenderGraphNode {
    pub fn new(node_type: WorldRenderGraphNodeType) -> Self {
        Self {
            node_type,
            position: math::Matrix4x4::identity(),
            rotation: math::Matrix4x4::identity(),
        }
    }

    pub fn set_position(&mut self, to_set: math::Matrix4x4) {
        self.position = to_set;
    }

    pub fn set_rotation(&mut self, to_set: math::Matrix4x4) {
        self.rotation = to_set;
    }
    
    
    pub fn draw(
        &mut self,
        display: &mut glium::Display,
        target: &mut glium::Frame,
        perspective: math::Matrix4x4,
        view: math::Matrix4x4,
        light_direction: math::Vector4
    ) {
        match &mut self.node_type {
            WorldRenderGraphNodeType::Mesh(mesh_node) => {
                mesh_node.draw(display, target, self.rotation * self.position, perspective, view, light_direction);
            },
        }
    }
}


pub enum WorldRenderGraphNodeType {
    Mesh(MeshGraphNode)
}

pub struct MeshGraphNode {
    mesh: mesh::Mesh,
    glium_mesh: Option<GliumMesh>
}

impl MeshGraphNode {
    pub fn new(mesh: mesh::Mesh) -> Self {
        Self {
            mesh,
            glium_mesh: None
        }
    }

    fn draw(
        &mut self,
        display: &mut glium::Display,
        target: &mut glium::Frame,
        orientation: math::Matrix4x4,
        perspective: math::Matrix4x4,
        view: math::Matrix4x4,
        light_direction: Vector4
    ) { 
        if self.glium_mesh.is_none() {
            self.glium_mesh = Some(GliumMesh::new(display, &self.mesh))
        }

        self.glium_mesh
            .as_ref()
            .unwrap()
            .draw(target, orientation, perspective, view, light_direction);
    }
}

pub struct GliumMesh {
    vertices: glium::vertex::VertexBuffer<Vertex>,
    normal_map: glium::Texture2d,
    shader_program: glium::Program,
}

impl GliumMesh {
    pub fn new(display: &glium::Display, mesh: &mesh::Mesh) -> Self {
        let vertex_data: Vec<Vertex> = mesh.into(); 
        Self {
            vertices: glium::vertex::VertexBuffer::immutable(display, &vertex_data).unwrap(),
            normal_map: textures::get_normal_texture(&display),
            shader_program: shaders::create_shader_program(&display).unwrap()
        }
    }

    fn draw(&self, target: &mut glium::Frame, orientation: math::Matrix4x4, perspective: math::Matrix4x4, view: math::Matrix4x4, light_direction: Vector4) {
        let model: [[f32; 4]; 4] = orientation.into_column_major();
        let view: [[f32; 4]; 4] = view.into_column_major();
        let perspective: [[f32; 4]; 4] = perspective.into_column_major();
        let light:[f32; 3] = light_direction.into();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            polygon_mode: glium::PolygonMode::Fill,
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target
            .draw(
                &self.vertices,
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.shader_program,
                &glium::uniform! {
                    model: model,
                    view: view,
                    perspective: perspective,
                    u_light: light,
                    normal_tex: &self.normal_map
                },
                &params)
            .unwrap();
    }
}

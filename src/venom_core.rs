use nalgebra::{Vector2, Vector3};

pub type Scalar = f32;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: (Scalar, Scalar, Scalar),
    pub normal: (Scalar, Scalar, Scalar),
    pub uv: (Scalar, Scalar),
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub models: Vec<Model>,
}

#[derive(Debug, Clone)]
pub struct OctreeNode {
    pub bounds: BoundingBox,
    pub vertices: Vec<Vertex>,
    pub children: Option<Box<[OctreeNode; 8]>>, // 8 children for the octree
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: (Scalar, Scalar, Scalar),
    pub max: (Scalar, Scalar, Scalar),
}

impl BoundingBox {
    pub fn contains_point(&self, point: &(Scalar, Scalar, Scalar)) -> bool {
        point.0 >= self.min.0 && point.0 <= self.max.0 &&
            point.1 >= self.min.1 && point.1 <= self.max.1 &&
            point.2 >= self.min.2 && point.2 <= self.max.2
    }

    pub fn center(&self) -> (Scalar, Scalar, Scalar) {
        (
            (self.min.0 + self.max.0) / 2.0,
            (self.min.1 + self.max.1) / 2.0,
            (self.min.2 + self.max.2) / 2.0,
        )
    }
}

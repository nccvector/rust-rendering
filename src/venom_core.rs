use nalgebra::{Vector2, Vector3};

pub type Scalar = f32;

struct Vertex {
    position: Vector3<Scalar>,
    normal: Vector3<Scalar>,
    uv: Vector2<Scalar>,
}

struct Face {
    indices: [i32; 3],
}
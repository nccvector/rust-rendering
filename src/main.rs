#![allow(non_snake_case)]

use rust_embree::{GeometryType, CreateDevice, CreateTriangleGeometry, CreateScene, CommitScene, CastRay};

fn main() {
    let device = CreateDevice();
    let scene = CreateScene(&device);

    let verts: &[(f32, f32, f32)] = &[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0)
    ];
    let inds: &[(i32, i32, i32)] = &[
        (0, 1, 2)
    ];

    CreateTriangleGeometry(
        &device,
        &scene,
        verts,
        inds,
    );

    CommitScene(&scene);

    println!("{:?}", CastRay(&scene, (0.0, 0.0, -1.0, 0.0, 0.0, 1.0)));
}
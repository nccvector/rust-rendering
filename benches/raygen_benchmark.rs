#![allow(non_snake_case)]

use criterion::{criterion_group, criterion_main, Criterion};
use image::{ImageBuffer, Rgb};

#[path = "../src/vec_ops.rs"]
mod vec_ops;
#[path = "../src/camera.rs"]
mod camera;
#[path = "../src/renderer.rs"]
mod renderer;

use crate::renderer::{CreateEguiColorImageFromImageBuffer, Renderer};

fn bench_Raygen(c: &mut Criterion) {
    let mut renderer = Renderer::new();
    renderer.createDemoScene();     // use a benchmark scene in future

    let imageBuffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(640, 480);
    renderer.camera.resize(640.0, 480.0);
    c.bench_function("GetRays 640x480", |x| x.iter(|| { renderer.camera.getRays(); }));
    c.bench_function("RenderImageBuffer 640x480", |x| x.iter(|| { renderer.renderImageBuffer(); }));
    c.bench_function("ImageToEgui 640x480", |x| x.iter(|| { CreateEguiColorImageFromImageBuffer(&imageBuffer); }));

    let imageBuffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(800, 600);
    renderer.camera.resize(800.0, 600.0);
    c.bench_function("GetRays 800x600", |x| x.iter(|| { renderer.camera.getRays(); }));
    c.bench_function("RenderImageBuffer 800x600", |x| x.iter(|| { renderer.renderImageBuffer(); }));
    c.bench_function("ImageToEgui 800x600", |x| x.iter(|| { CreateEguiColorImageFromImageBuffer(&imageBuffer); }));

    let imageBuffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(1280, 720);
    renderer.camera.resize(1280.0, 720.0);
    c.bench_function("GetRays 1280x720", |x| x.iter(|| { renderer.camera.getRays(); }));
    c.bench_function("RenderImageBuffer 1280x720", |x| x.iter(|| { renderer.renderImageBuffer(); }));
    c.bench_function("ImageToEgui 1280x720", |x| x.iter(|| { CreateEguiColorImageFromImageBuffer(&imageBuffer); }));

    let imageBuffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(1920, 1080);
    renderer.camera.resize(1920.0, 1080.0);
    c.bench_function("GetRays 1920x1080", |x| x.iter(|| { renderer.camera.getRays(); }));
    c.bench_function("RenderImageBuffer 1920x1080", |x| x.iter(|| { renderer.renderImageBuffer(); }));
    c.bench_function("ImageToEgui 1920x1080", |x| x.iter(|| { CreateEguiColorImageFromImageBuffer(&imageBuffer); }));
}

criterion_group!(benches, bench_Raygen);

criterion_main!(benches);

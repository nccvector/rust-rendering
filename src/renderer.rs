use eframe::egui;
use eframe::egui::{Color32, ColorImage, TextureHandle};
use image::{ImageBuffer, Rgb};
use nalgebra::Matrix4;
use rust_embree::{CastRay, CommitScene, CreateDevice, CreateScene, CreateSphereGeometry, CreateTriangleGeometry, EmbreeDevice, EmbreeScene};
use crate::camera::Camera;

use russimp::node::Node;
use russimp::property::Property;
use russimp::scene::PostProcess;
use russimp::{property::PropertyStore, scene::Scene};


pub fn CreateEguiColorImageFromImageBuffer(imageBuffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ColorImage {
    let pixels: Vec<Color32> = imageBuffer.pixels().map(|p| {
        let [r, g, b] = p.0;
        Color32::from_rgba_premultiplied(r, g, b, 255)
    }).collect();

    ColorImage {
        size: [imageBuffer.width() as usize, imageBuffer.height() as usize],
        pixels,
    }
}

fn LoadEguiTextureFromImageBuffer(ctx: &egui::Context, imageBuffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> TextureHandle {
    let eguiColorImage = CreateEguiColorImageFromImageBuffer(imageBuffer);
    ctx.load_texture("", eguiColorImage, Default::default())
}

pub struct Renderer {
    pub renderTexture: Option<TextureHandle>,
    pub camera: Camera,
    device: EmbreeDevice,
    scene: EmbreeScene,
}

impl Renderer {
    pub fn new() -> Self {
        let device = CreateDevice();
        let scene = CreateScene(&device);
        let camera = Camera::new(Matrix4::<f32>::identity(), 65.0, 640.0, 480.0);

        Self {
            renderTexture: None,
            camera,
            device,
            scene,
        }
    }

    pub fn createDemoScene(&mut self) {
        // Quad vertices and indices
        let vertices: &[(f32, f32, f32)] = &[
            (-1.5, -1.5, 0.0),
            (1.5, -1.5, 0.0),
            (1.5, 1.5, 0.0),
            (-1.5, 1.5, 0.0)
        ];

        let indices: &[(u32, u32, u32)] = &[
            (0, 1, 2),
            (0, 2, 3),
        ];

        CreateTriangleGeometry(
            &self.device,
            &self.scene,
            vertices,
            indices,
        );

        CreateSphereGeometry(
            &self.device,
            &self.scene,
            (0.0, 0.0, 0.0),
            1.0,
        );

        CommitScene(&self.scene);
    }

    pub fn loadScene(&mut self) {
        // load sponza
        let props: PropertyStore = PropertyStore::default();

        let scene = Scene::from_file_with_props(
            "/home/mujin/workdesk/Sponza/sponza.obj",
            vec![
                PostProcess::Triangulate,
                PostProcess::GenerateSmoothNormals,
                // PostProcess::FlipUVs,
                // PostProcess::FlipWindingOrder,
                PostProcess::JoinIdenticalVertices,
                PostProcess::OptimizeGraph,
            ],
            &props,
        )
            .unwrap();

        for mesh in scene.meshes {
            let mut vertices: Vec<(f32, f32, f32)> = vec![];
            let mut indices: Vec<(u32, u32, u32)> = vec![];
            for vertex in mesh.vertices.iter() {
                vertices.push((vertex.x, vertex.y, vertex.z));
            }
            for face in mesh.faces {
                indices.push((face.0[0], face.0[1], face.0[2]));
            }

            CreateTriangleGeometry(&self.device, &self.scene, &vertices, &indices);
            CommitScene(&self.scene);
        }
    }

    pub fn renderImageBuffer(&mut self) -> ImageBuffer::<Rgb<u8>, Vec<u8>> {
        let mut imageBuffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.camera.imageWidth as u32, self.camera.imageHeight as u32);

        let mut rays = self.camera.getRays();
        let totalNumRays = rays.iter().count();

        for y in 0..self.camera.imageHeight as u32 {
            for x in 0..self.camera.imageWidth as u32 {
                let i: usize = (y * self.camera.imageWidth as u32 + x) as usize;

                // Reverse rays to rotate the final image 180 degrees
                let rayHit = CastRay(&self.scene, rays[totalNumRays - 1 - i]);

                let mut color = Rgb([0, 0, 0]);
                if rayHit.is_some() {
                    let hit = rayHit.unwrap().hit;

                    color = Rgb([
                        (255.0 * hit.Ng_x) as u8,
                        (255.0 * hit.Ng_y) as u8,
                        (255.0 * hit.Ng_z) as u8,
                    ]);
                }

                *imageBuffer.get_pixel_mut(x, y) = color;
            }
        }

        imageBuffer
    }

    pub fn renderNormalsToTexture(&mut self, ctx: Option<&egui::Context>) {
        let imageBuffer = self.renderImageBuffer();

        if ctx.is_some() {
            self.renderTexture = Some(LoadEguiTextureFromImageBuffer(&ctx.unwrap(), &imageBuffer));
        }
    }
}
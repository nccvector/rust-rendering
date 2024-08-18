use eframe::egui;
use eframe::egui::{Color32, ColorImage, TextureHandle};
use image::{ImageBuffer, Rgb};
use nalgebra::Matrix4;
use rust_embree::{CastRay, CommitScene, CreateDevice, CreateScene, CreateSphereGeometry, CreateTriangleGeometry, Device, Scene};
use crate::camera::Camera;

fn LoadEguiTextureFromImageBuffer(ctx: &egui::Context, imageBuffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> TextureHandle {
    let pixels: Vec<Color32> = imageBuffer.pixels().map(|p| {
        let [r, g, b] = p.0;
        Color32::from_rgba_premultiplied(r, g, b, 255)
    }).collect();

    let colorImage = ColorImage {
        size: [imageBuffer.width() as usize, imageBuffer.height() as usize],
        pixels: pixels,
    };

    ctx.load_texture("", colorImage, Default::default())
}

pub struct Renderer {
    pub renderTexture: Option<TextureHandle>,
    pub camera: Camera,
    device: Device,
    scene: Scene,
}

impl Renderer {
    pub fn new() -> Self {
        let device = CreateDevice();
        let scene = CreateScene(&device);
        let camera = Camera::new(Matrix4::<f32>::identity(), 65.0, 640.0, 480.0);

        // Quad vertices and indices
        let vertices: &[(f32, f32, f32)] = &[
            (-1.5, -1.5, 0.0),
            (1.5, -1.5, 0.0),
            (1.5, 1.5, 0.0),
            (-1.5, 1.5, 0.0)
        ];
        let indices: &[(i32, i32, i32)] = &[
            (0, 1, 2),
            (0, 2, 3),
        ];

        CreateTriangleGeometry(
            &device,
            &scene,
            vertices,
            indices,
        );

        CreateSphereGeometry(
            &device,
            &scene,
            (0.0, 0.0, 0.0),
            1.0,
        );

        CommitScene(&scene);

        Self {
            renderTexture: None,
            camera,
            device,
            scene,
        }
    }

    pub fn renderToRenderTexture(&mut self, ctx: &egui::Context) {
        let mut imageBuffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.camera.imageWidth as u32, self.camera.imageHeight as u32);

        let rays = self.camera.getRays();

        for y in (0..self.camera.imageHeight as u32) {
            for x in (0..self.camera.imageWidth as u32) {
                let i: usize = (y * self.camera.imageWidth as u32 + x) as usize;

                let rayhit = CastRay(&self.scene, rays[i]);
                let mut color = Rgb([0, 0, 0]);
                if rayhit.is_some() {
                    color = Rgb([255, 0, 0]);
                }
                *imageBuffer.get_pixel_mut(x, y) = color;
            }
        }

        self.renderTexture = Some(LoadEguiTextureFromImageBuffer(ctx, &imageBuffer));
    }
}
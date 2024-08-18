#![allow(non_snake_case)]

use eframe::{egui, App, NativeOptions};
use eframe::egui::{Color32, ColorImage, Image, Rect, TextureHandle, Ui};
use eframe::egui::CentralPanel;
use nalgebra::{Matrix3, Matrix4, Vector2, Vector3, Vector4};
use image::{ImageBuffer, Rgb};
use rust_embree::{CreateDevice, CreateTriangleGeometry, CreateScene, CommitScene, CastRay, CreateSphereGeometry, Device, Scene};

const VERTICAL_FOV: f32 = 45.0;
const IMAGE_WIDTH: u32 = 640;
const IMAGE_HEIGHT: u32 = 480;

mod vec_ops;
mod camera;

use vec_ops::*;


fn ComputeCameraMatrix(verticalFOVDegrees: f32, imageWidth: f32, imageHeight: f32) -> Matrix3<f32> {
    // Convert vertical FOV from degrees to radians
    let verticalFOVRadians = verticalFOVDegrees.to_radians();
    let aspectRatio = imageWidth / imageHeight;
    let horizontalFOVRadians = aspectRatio * verticalFOVRadians;


    let fx = imageWidth / (2.0 * (horizontalFOVRadians / 2.0).tan());
    let fy = imageHeight / (2.0 * (verticalFOVRadians / 2.0).tan());

    // Compute the image center
    let cx = (imageWidth - 1.0) / 2.0;
    let cy = (imageHeight - 1.0) / 2.0;

    // Intrinsic camera matrix
    let cameraMatrix: Matrix3<f32> = Matrix3::new(
        fx, 0.0, cx,
        0.0, fy, cy,
        0.0, 0.0, 1.0,
    );

    cameraMatrix
}

fn GenerateHomogenousPixelCoordinates(imageWidth: u32, imageHeight: u32) -> Vec<Vector3<f32>> {
    let mut pixels = Vec::new();

    for y in 0..imageHeight {
        for x in 0..imageWidth {
            pixels.push(Vector3::new(x as f32, y as f32, 1.0));
        }
    }

    pixels
}

struct Camera {
    transform: Matrix4<f32>,
    cameraMatrix: Matrix3<f32>,
    cameraMatrixInverse: Matrix3<f32>,
    imageWidth: f32,
    imageHeight: f32,
}

impl Camera {
    fn new(transform: Matrix4<f32>, verticalFov: f32, imageWidth: f32, imageHeight: f32) -> Self {
        let _cameraMatrix = ComputeCameraMatrix(verticalFov, imageWidth, imageHeight);
        Self {
            transform,
            cameraMatrix: _cameraMatrix,
            cameraMatrixInverse: _cameraMatrix.try_inverse().unwrap(),
            imageWidth,
            imageHeight,
        }
    }

    fn getRays(&mut self) -> Vec<(f32, f32, f32, f32, f32, f32)> {
        // Get pixel coordinates
        let pixelCoordsHomo = GenerateHomogenousPixelCoordinates(self.imageWidth as u32, self.imageHeight as u32);

        // Get rays for all pixels
        // let rayOrigins =
        let rayDirections = pixelCoordsHomo.transform(&self.cameraMatrixInverse).normalize().mul_scalar(-1.0);
        let rayOrigins = rayDirections.zeros().add_vector(Vector3::<f32>::new(0.0, 0.0, 10.0));

        let mut vec: Vec<(f32, f32, f32, f32, f32, f32)> = Vec::with_capacity(rayDirections.len());

        for (origin, direction) in rayOrigins.iter().zip(rayDirections.iter()) {
            vec.push((
                origin[0], origin[1], origin[2],
                direction[0], direction[1], direction[2]
            ));
        }

        vec
    }
}

fn LoadEguiTextureFromImageBuffer(ctx: &egui::Context, imageBuffer: &ImageBuffer::<Rgb<u8>, Vec<u8>>) -> TextureHandle {
    let pixels: Vec<Color32> = imageBuffer.pixels().map(|p| {
        let [r, g, b] = p.0;
        Color32::from_rgba_premultiplied(r, g, b, 255)
    }).collect();

    let colorImage = ColorImage {
        size: [IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize],
        pixels: pixels,
    };

    ctx.load_texture("", colorImage, Default::default())
}

struct Renderer {
    renderTexture: Option<TextureHandle>,
    renderTextureSize: (usize, usize),
    camera: Camera,
    device: Device,
    scene: Scene,
}

impl Renderer {
    fn new() -> Self {
        let device = CreateDevice();
        let scene = CreateScene(&device);
        let camera = Camera::new(Matrix4::<f32>::identity(), 65.0, IMAGE_WIDTH as f32, IMAGE_HEIGHT as f32);

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
            renderTextureSize: (IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize),
            camera,
            device,
            scene,
        }
    }

    fn renderToRenderTexture(&mut self, ctx: &egui::Context) {
        let mut imageBuffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(IMAGE_WIDTH, IMAGE_HEIGHT);

        let rays = self.camera.getRays();

        for y in (0..IMAGE_HEIGHT) {
            for x in (0..IMAGE_WIDTH) {
                let i: usize = (y * IMAGE_WIDTH + x) as usize;

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

impl App for Renderer {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Render every frame
        self.renderToRenderTexture(ctx);

        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("Hello bro");
            if let Some(ref texture) = self.renderTexture {
                let img = Image::from_texture(texture);
                img.paint_at(ui, Rect {
                    min: egui::pos2(0.0, 0.0),
                    max: egui::pos2(self.renderTextureSize.0 as f32, self.renderTextureSize.1 as f32),
                });
            }
        });
    }
}

fn main() {
    let options = NativeOptions::default();

    eframe::run_native(
        "",
        options,
        Box::new(|cc| Ok(Box::new(Renderer::new()))),
    ).unwrap();
}
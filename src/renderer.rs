use std::cell::RefCell;
use std::ops::DerefMut;
use std::ptr::null;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use eframe::egui;
use eframe::egui::{Color32, ColorImage, TextureHandle};
use image::{ImageBuffer, Rgb};
use nalgebra::{Matrix4, Rotation3, Vector3};
use rust_embree::{CastRay, CreateSphereGeometry, CreateTriangleGeometry};

use crate::camera::Camera;

use russimp::node::Node;
use russimp::property::Property;
use russimp::scene::PostProcess;
use russimp::{property::PropertyStore, scene::Scene};
use rust_embree::bindings_embree::{rtcCommitGeometry, rtcCommitScene, rtcNewDevice, rtcNewScene, RTCDevice, RTCDeviceTy, RTCScene, RTCSceneTy};


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
    imageBuffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    device: Arc<RTCDeviceTy>,
    scene: Arc<RTCSceneTy>,
}

impl Renderer {
    pub fn new() -> Self {
        let device = unsafe { Arc::from_raw(rtcNewDevice(null())) };
        let scene = unsafe { Arc::from_raw(rtcNewScene(Arc::into_raw(Arc::clone(&device)) as RTCDevice)) };

        Self {
            renderTexture: None,
            imageBuffer: ImageBuffer::default(),
            camera: Camera::new(Matrix4::<f32>::identity(), 45.0, 640.0, 480.0),
            device: device,
            scene: scene,
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

        let device = Arc::into_raw(Arc::clone(&self.device)) as RTCDevice;
        let scene = Arc::into_raw(Arc::clone(&self.scene)) as RTCScene;

        CreateTriangleGeometry(
            device,
            scene,
            vertices,
            indices,
        );

        CreateSphereGeometry(
            device,
            scene,
            (0.0, 0.0, 0.0),
            1.0,
        );

        unsafe { rtcCommitScene(scene) };
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

            CreateTriangleGeometry(
                Arc::into_raw(self.device.clone()) as RTCDevice,
                Arc::into_raw(self.scene.clone()) as RTCScene,
                &vertices,
                &indices,
            );

            unsafe { rtcCommitScene(Arc::into_raw(self.scene.clone()) as RTCScene) };
        }
    }

    fn renderChunk(imageBuffer: Arc<Mutex<ImageBuffer<Rgb<u8>, Vec<u8>>>>, scene: Arc<RTCSceneTy>, camera: Arc<Camera>, rays: &Vec<(f32, f32, f32, f32, f32, f32)>, chunk: (u32, u32, u32, u32)) {
        let totalNumRays = rays.iter().count();

        let (minX, minY, maxX, maxY) = chunk;
        for y in minY..maxY {
            for x in minX..maxX {
                let i: usize = (y * camera.imageWidth as u32 + x) as usize;

                // Reverse rays to rotate the final image 180 degrees
                let rayHit = CastRay(Arc::into_raw(scene.clone()) as RTCScene, rays[totalNumRays - 1 - i]);

                let mut color = Rgb([255, 0, 255]);
                if rayHit.is_some() {
                    let hit = rayHit.unwrap().hit;

                    color = Rgb([
                        (255.0 * hit.Ng_x) as u8,
                        (255.0 * hit.Ng_y) as u8,
                        (255.0 * hit.Ng_z) as u8,
                    ]);
                }

                let mut buffer = imageBuffer.lock().unwrap();
                *buffer.get_pixel_mut(x, y) = color;
            }
        }
    }


    pub fn renderImageBuffer(&mut self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let imageBuffer = Arc::new(Mutex::new(ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.camera.imageWidth as u32, self.camera.imageHeight as u32)));
        let imageWidth = self.imageBuffer.width() as u32;
        let imageHeight = self.imageBuffer.height() as u32;

        let rays = self.camera.getTransformedRays();
        let totalNumRays = rays.iter().count();

        // // Closure with `renderChunk` logic
        // for y in 0..self.imageBuffer.height() {
        //     for x in 0..self.imageBuffer.width() {
        //         let ray_index = (y * self.imageBuffer.width() + x) as usize;
        //         if let Some(rayHit) = CastRay(&(Arc::into_raw(self.scene.clone()) as RTCScene), rays[totalNumRays - 1 - ray_index]) {
        //             let hit = rayHit.hit;
        //             let color = Rgb([
        //                 (255.0 * hit.Ng_x) as u8,
        //                 (255.0 * hit.Ng_y) as u8,
        //                 (255.0 * hit.Ng_z) as u8,
        //             ]);
        //             *self.imageBuffer.get_pixel_mut(x, y) = color;
        //         }
        //     }
        // }
        //
        // self.imageBuffer.clone()


        let numThreads: u32 = 10;
        let chunkWidth = self.camera.imageWidth as u32 / numThreads;

        let mut handles = vec![];

        for i in 0..numThreads {
            let imageBuf = Arc::clone(&imageBuffer);
            let camera = Arc::new(self.camera.clone());
            let scene = Arc::clone(&self.scene);
            let rays = rays.clone();

            let handle = thread::spawn(move || {
                Renderer::renderChunk(
                    imageBuf,
                    Arc::clone(&scene),
                    Arc::clone(&camera),
                    &rays,
                    (i * chunkWidth, 0, (i + 1) * chunkWidth, imageHeight)
                );
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        self.imageBuffer.clone()
    }


    pub fn renderNormalsToTexture(&mut self, ctx: Option<&egui::Context>) {
        let imageBuffer = self.renderImageBuffer();

        if ctx.is_some() {
            self.renderTexture = Some(LoadEguiTextureFromImageBuffer(&ctx.unwrap(), &imageBuffer));
        }
    }
}

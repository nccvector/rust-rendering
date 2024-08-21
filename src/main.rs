#![allow(non_snake_case)]

use raylib::color::Color as RaylibColor;
use eframe::{egui, App, NativeOptions};
use eframe::egui::{Image, Rect, Ui};
use eframe::egui::CentralPanel;
use eframe::egui::ImageData::{Color as EguiColor, Color};
use raylib::ffi::{DrawCube, GenMeshCylinder};
use raylib::prelude::*;

mod vec_ops;
mod camera;
mod renderer;

use crate::renderer::Renderer;

impl App for Renderer {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            let size = ui.min_rect().max;

            // Render every frame
            self.camera.resize(size.x, size.y);
            self.renderToRenderTexture(ctx);

            ui.label("Hello bro");

            if let Some(ref texture) = self.renderTexture {
                let img = Image::from_texture(texture);
                img.paint_at(ui, Rect {
                    min: egui::pos2(ui.min_rect().min.x, ui.min_rect().min.y),
                    max: egui::pos2(self.camera.imageWidth, self.camera.imageHeight),
                });
            }
        });
    }
}

fn main() {
    let mut renderer = Renderer::new();

    // let (mut rl, thread) = raylib::init().size(800, 600).title("Hello").build();
    //
    // let mut camera = Camera3D::perspective(
    //     Vector3::new(4.0, 2.0, 4.0),
    //     Vector3::new(0.0, 1.8, 0.0),
    //     Vector3::new(0.0, 1.0, 0.0),
    //     60.0,
    // );
    // camera.position = Vector3::new(3.5, 0.65, -3.0);
    // camera.target = Vector3::new(0.0, 0.0, 0.0);
    //
    // let meshCyl = unsafe { WeakMesh::from_raw(GenMeshCylinder(0.025, 1.0, 10)) };
    // let modelCyl = rl.load_model_from_mesh(&thread, meshCyl).unwrap();
    //
    // rl.set_target_fps(60);
    //
    // while !rl.window_should_close() {
    //     let mut d = rl.begin_drawing(&thread);
    //     d.clear_background(RaylibColor::WHITE);
    //
    //     d.update_camera(&mut camera, CameraMode::CAMERA_FIRST_PERSON);
    //
    //     {
    //         let mut d2 = d.begin_mode3D(camera);
    //
    //         d2.draw_model_ex(&modelCyl, Vector3::zero(), Vector3::forward(), -90.0, Vector3::one(), RaylibColor::RED);
    //         d2.draw_model_ex(&modelCyl, Vector3::zero(), Vector3::forward(), 0.0, Vector3::one(), RaylibColor::GREEN);
    //         d2.draw_model_ex(&modelCyl, Vector3::zero(), Vector3::right(), 90.0, Vector3::one(), RaylibColor::BLUE);
    //
    //         let rays = renderer.camera.getRays();
    //         for mut ray in rays {
    //             ray.3 /= ray.5.abs();
    //             ray.4 /= ray.5.abs();
    //             ray.5 /= ray.5.abs();
    //             d2.draw_line_3D(
    //                 Vector3::new(ray.0, ray.1, ray.2),
    //                 Vector3::new(ray.0 + ray.3, ray.1 + ray.4, ray.2 + ray.5),
    //                 RaylibColor::new(100, 100, 100, 100),
    //             );
    //
    //             d2.draw_circle_3D(
    //                 Vector3::new(ray.0 + ray.3, ray.1 + ray.4, ray.2 + ray.5),
    //                 0.005,
    //                 Vector3::forward(),
    //                 0.0,
    //                 RaylibColor::RED,
    //             );
    //         }
    //     }
    // }

    let options = NativeOptions::default();
    eframe::run_native(
        "",
        options,
        Box::new(|cc| Ok(Box::new(renderer))),
    ).unwrap();
}
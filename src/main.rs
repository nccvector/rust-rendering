#![allow(non_snake_case)]

use eframe::{egui, App, NativeOptions};
use eframe::egui::{Image, Rect, Ui};
use eframe::egui::CentralPanel;

const VERTICAL_FOV: f32 = 45.0;
const IMAGE_WIDTH: u32 = 640;
const IMAGE_HEIGHT: u32 = 480;

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
    let options = NativeOptions::default();

    eframe::run_native(
        "",
        options,
        Box::new(|cc| Ok(Box::new(Renderer::new()))),
    ).unwrap();
}
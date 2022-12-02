#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;

mod vec;
use vec::prelude::*;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Vector Calculator",
        options,
        Box::new(|_cc| Box::new(VectorApp::default()))
    );
}

struct VectorApp {
    vec_a_str: String,
    vec_b_str: String,
}

impl Default for VectorApp {
    fn default() -> Self {
        Self {
            vec_a_str: "0.2,0.4,0.2".to_owned(),
            vec_b_str: "2,5,1".to_owned()
        }
    }
}

impl eframe::App for VectorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Vector A");


            ui.horizontal(|ui| {
                ui.label("Enter Vector: ");
                ui.text_edit_singleline(&mut self.vec_a_str);

                if self.vec_a_str.len() == 0 {
                    self.vec_a_str = "0, 0, 0".to_owned();
                }
            });


            let vec_a: Vec3 = Vec3::from_str(&self.vec_a_str);
            let vec_a_norm: Vec3 = vec_a.normalize();
            ui.label(format!("Vector A: <{}, {}, {}>", vec_a.x, vec_a.y, vec_a.z));
            ui.label(format!("Vector A Magnitude: {}", vec_a.magnitude()));
            ui.label(format!("Vector A Normalized: <{}, {}, {}>", vec_a_norm.x, vec_a_norm.y, vec_a_norm.z));

            ui.heading("Vector B");
            ui.horizontal(|ui| {
                ui.label("Enter Vector: ");
                ui.text_edit_singleline(&mut self.vec_b_str).lost_focus();

                if self.vec_b_str.len() == 0 {
                    self.vec_b_str = "0, 0, 0".to_owned();
                }
            });
            let vec_b: Vec3 = Vec3::from_str(&self.vec_b_str);
            let vec_b_norm: Vec3 = vec_b.normalize();
            ui.label(format!("Vector B: <{}, {}, {}>", vec_b.x, vec_b.y, vec_b.z));
            ui.label(format!("Vector B Magnitude: {}", vec_b.magnitude()));
            ui.label(format!("Vector B Normalized: <{}, {}, {}>", vec_b_norm.x, vec_b_norm.y, vec_b_norm.z));

            ui.heading("Dot Product");
            ui.label(format!("Vec A Dot Vec B: {}", math::dot(vec_a, vec_b)));
            ui.label(format!("Angle: {} radians", math::get_angle(vec_a, vec_b)));

            ui.heading("Cross Product");
            let cross = math::cross(vec_a, vec_b);
            ui.label(format!("Vec A Cross Vec B: <{}, {}, {}>", cross.x, cross.y, cross.z));


            /* 
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            
            if ui.button("Click each year AAAA").clicked() {
                self.age += 1;
            }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            */
        });
    }
}
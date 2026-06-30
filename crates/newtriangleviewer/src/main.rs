use eframe::egui;
//use eframe::egui::Response;
//use eframe::egui::Vec2;
//use egui::{Color32, Pos2};
//use egui_plot::MarkerShape;
//use egui_plot::Plot;
//use egui_plot::PlotPoints;
//use egui_plot::Points;
//use std::collections::VecDeque;
//use std::ops;

fn main() -> eframe::Result {
    eframe::run_native(
        "New Triangle Viewer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApplication::default()))),
    )
}

pub struct MyApplication {
    counter: i32,
}

impl Default for MyApplication {
    fn default() -> Self {
        Self {
            counter: 107,
        }
    }
}

impl eframe::App for MyApplication {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Hello");

        if ui.button("Click me").clicked() {
            self.counter += 1;
        }

        ui.label(format!("Count: {}", self.counter));
    }
}

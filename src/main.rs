#![windows_subsystem = "windows"]
mod bulkgui;

use eframe::egui;
use egui_extras;
use bulkgui::bulk_gui::*;

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(egui::Vec2 { x: 1280.0, y: 800.0 }),
        min_window_size: Some(egui::Vec2 { x: 1280.0, y: 800.0 }),
        //icon_data: Some(),  <- Load Icon Data Somehow!!
        ..Default::default()
    };
    eframe::run_native(
        "Bulk Rename Utility",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<BulkGui>::default()
        }),
    )
}

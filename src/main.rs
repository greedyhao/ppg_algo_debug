#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod demo;
mod file2data;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Box::new(plot_ppg::LineDemo::default());
    // let app = ppg_gui_lib::TemplateApp::default();
    let app = demo::DemoApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

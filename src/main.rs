#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod demo;
mod file2data;
use crate::file2data::file2data::*;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Box::new(plot_ppg::LineDemo::default());
    // let app = ppg_gui_lib::TemplateApp::default();
    let cols = get_cols_vec_from_file("ppg_data.txt".to_string());
    
    let (cnt, v) = cols;
    let col0 = get_col_from_cols_vec(0, &v);
    let col1 = get_col_from_cols_vec(1, &v);

    println!("col is {} {:?} {:?}", cnt, col0[0], col1[0]);
    let app = demo::DemoApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

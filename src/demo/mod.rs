mod app;
pub mod demo_app_windows;
pub mod plot_demo;
pub mod widget_gallery;

pub use app::DemoApp;
pub use demo_app_windows::DemoWindows;
pub use widget_gallery::WidgetGallery;
// pub use {
//     app::DemoApp, demo_app_windows::DemoWindows, misc_demo_window::MiscDemoWindow,
//     widget_gallery::WidgetGallery,
// };

/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut eframe::egui::Ui);
}

/// Something to view
pub trait Demo {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &eframe::egui::CtxRef, open: &mut bool);
}

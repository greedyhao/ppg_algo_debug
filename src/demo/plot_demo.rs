use eframe::*;
use eframe::egui::*;
use plot::{
    Arrows, Corner, HLine, Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, Points, Polygon,
    Text, VLine, Value, Values,
};
// use std::f64::consts::TAU;

#[derive(PartialEq)]
struct LineDemo {
    animate: bool,
    time: f64,
    square: bool,
    proportional: bool,
    line_style: LineStyle,
    open: bool,
}

impl Default for LineDemo {
    fn default() -> Self {
        Self {
            animate: !cfg!(debug_assertions),
            time: 0.0,
            square: false,
            proportional: true,
            line_style: LineStyle::Solid,
            open: true,
        }
    }
}

impl LineDemo {
    pub fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            animate,
            time: _,
            square,
            proportional,
            line_style,
            ..
        } = self;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.style_mut().wrap = Some(false);
                ui.checkbox(animate, "animate");
                ui.checkbox(square, "square view")
                    .on_hover_text("Always keep the viewport square.");
                ui.checkbox(proportional, "Proportional data axes")
                    .on_hover_text("Tick are the same size on both axes.");
            });

            ui.vertical(|ui| {
                ComboBox::from_label("Line style")
                    .selected_text(line_style.to_string())
                    .show_ui(ui, |ui| {
                        [
                            LineStyle::Solid,
                            LineStyle::dashed_dense(),
                            LineStyle::dashed_loose(),
                            LineStyle::dotted_dense(),
                            LineStyle::dotted_loose(),
                        ]
                        .iter()
                        .for_each(|style| {
                            ui.selectable_value(line_style, *style, style.to_string());
                        });
                    });
            });
        });
    }

    fn sample(&self) -> Line {
        Line::new(Values::from_parametric_callback(
            move |t| (t, 1.0),
            0.0..=5.0,
            256
        ))
        .color(Color32::from_rgb(200, 100, 100))
        .style(self.line_style)
        .name("sample")
    }
}

impl Widget for &mut LineDemo {
    fn ui(self, ui: &mut Ui) -> Response {
        self.options_ui(ui);
        if self.animate {
            ui.ctx().request_repaint();
            self.time += ui.input().unstable_dt.at_most(1.0 / 30.0) as f64;
        };
        let mut plot = Plot::new("lines_demo")
            .line(self.sample())
            .legend(Legend::default());
        if self.square {
            plot = plot.view_aspect(1.0);
        }
        if self.proportional {
            plot = plot.data_aspect(1.0);
        }
        ui.add(plot)
    }
}

#[derive(PartialEq, Default)]
pub struct PlotDemo {
    line_demo: LineDemo,
    string: String,
    // open_panel: Panel,
}

impl super::Demo for PlotDemo {
    fn name(&self) -> &'static str {
        "Plot"
    }

    fn show(&mut self, ctx: &eframe::egui::CtxRef, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .default_size(vec2(400.0, 400.0))
            .scroll(false)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}

use rfd::FileDialog;
use crate::demo::View;
impl super::View for PlotDemo {
    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            egui::reset_button(ui, self);
            ui.collapsing("Instructions", |ui| {
                ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                if cfg!(target_arch = "wasm32") {
                    ui.label("Zoom with ctrl / ⌘ + mouse wheel, or with pinch gesture.");
                } else if cfg!(target_os = "macos") {
                    ui.label("Zoom with ctrl / ⌘ + scroll.");
                } else {
                    ui.label("Zoom with ctrl + scroll.");
                }
                ui.label("Reset view with double-click.");
            });
        });

        ui.separator();
        let string = &mut self.string;
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(string).hint_text("Write something here"));
            if ui.add(Button::new("select file")).clicked() {
                let files = FileDialog::new()
                    .add_filter("text", &["txt"])
                    .set_directory("")
                    .pick_file();
                *string = files.unwrap().into_os_string().into_string().unwrap();
            }
        });
        ui.separator();
        // ui.horizontal(|ui| {
            // ui.selectable_value(&mut self.open_panel, Panel::Lines, "Lines");
            // ui.selectable_value(&mut self.open_panel, Panel::Markers, "Markers");
            // ui.selectable_value(&mut self.open_panel, Panel::Legend, "Legend");
            // ui.selectable_value(&mut self.open_panel, Panel::Items, "Items");
        // });
        // ui.separator();

        // match self.open_panel {
        //     Panel::Lines => {
                ui.add(&mut self.line_demo);
        //     }
        //     Panel::Markers => {
        //         ui.add(&mut self.marker_demo);
        //     }
        //     Panel::Legend => {
        //         ui.add(&mut self.legend_demo);
        //     }
        //     Panel::Items => {
        //         ui.add(&mut self.items_demo);
        //     }
        // }
    }
}


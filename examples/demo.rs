use eframe::egui;
use egui_circular_progress_bar::{CircularProgressBar, CircularProgressBarExt};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Circular Progress Bar Example",
        options,
        Box::new(|_cc| Ok(Box::new(ExampleApp::default()))),
    )
}

#[derive(Default)]
struct ExampleApp {
    progress: f32,
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-animate progress for demo
        self.progress = (self.progress + 0.01) % 1.0;
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Circular Progress Bar Examples");
            ui.separator();

            // Controls
            ui.horizontal(|ui| {
                ui.label("Progress:");
                ui.add(egui::Slider::new(&mut self.progress, 0.0..=1.0));
            });

            ui.separator();

            // Basic examples
            ui.heading("Basic Examples");

            ui.horizontal(|ui| {
                ui.label("Default size:");
                ui.circular_progress_bar(self.progress);
            });

            ui.horizontal(|ui| {
                ui.label("Custom size (60px):");
                ui.circular_progress_bar_with_size(self.progress, 60.0);
            });

            ui.horizontal(|ui| {
                ui.label("Large (100px):");
                ui.circular_progress_bar_with_size(self.progress, 100.0);
            });

            ui.separator();

            // Advanced examples
            ui.heading("Advanced Examples");

            ui.horizontal(|ui| {
                ui.label("With percentage text:");
                ui.add(
                    CircularProgressBar::new(self.progress)
                        .size(80.0)
                        .text(format!("{:.0}%", self.progress * 100.0)),
                );
            });

            ui.horizontal(|ui| {
                ui.label("With custom text:");
                ui.add(
                    CircularProgressBar::new(self.progress)
                        .size(80.0)
                        .text("Loading..."),
                );
            });

            ui.separator();

            // Multiple progress bars
            ui.heading("Multiple Progress Bars");

            ui.horizontal(|ui| {
                for i in 1..=5 {
                    let progress = (self.progress + (i as f32 * 0.2)) % 1.0;
                    ui.add(
                        CircularProgressBar::new(progress)
                            .size(40.0)
                            .text(format!("{}", i)),
                    );
                }
            });

            ui.separator();

            // File download simulation
            ui.heading("File Download Simulation");

            static mut DOWNLOAD_PROGRESS: f32 = 0.0;
            static mut DOWNLOADING: bool = false;

            unsafe {
                ui.horizontal(|ui| {
                    if ui.button("Start Download").clicked() {
                        DOWNLOADING = true;
                        DOWNLOAD_PROGRESS = 0.0;
                    }

                    if ui.button("Reset").clicked() {
                        DOWNLOADING = false;
                        DOWNLOAD_PROGRESS = 0.0;
                    }
                });

                if DOWNLOADING {
                    DOWNLOAD_PROGRESS += 0.005;
                    if DOWNLOAD_PROGRESS >= 1.0 {
                        DOWNLOAD_PROGRESS = 1.0;
                        DOWNLOADING = false;
                    }
                    ctx.request_repaint();
                }

                ui.horizontal(|ui| {
                    ui.add(
                        CircularProgressBar::new(DOWNLOAD_PROGRESS)
                            .size(160.0)
                            .text(if DOWNLOADING {
                                format!("Downloading... {:.0}%", DOWNLOAD_PROGRESS * 100.0)
                            } else if DOWNLOAD_PROGRESS >= 1.0 {
                                "Complete!".to_string()
                            } else {
                                "Ready".to_string()
                            }),
                    );

                    ui.vertical(|ui| {
                        ui.label(format!("Progress: {:.1}%", DOWNLOAD_PROGRESS * 100.0));
                        ui.label(if DOWNLOADING {
                            "Status: Downloading"
                        } else {
                            "Status: Idle"
                        });
                    });
                });
            }
        });
    }
}

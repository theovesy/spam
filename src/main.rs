mod spam;

use eframe::{epi, egui};

struct SpamApp {
    //resize: u32,
    template_width: u32,
    template_height: u32,
    aspect_ratio: f32,
    use_aspect: bool,
    template_path: String,
    template_success: bool,
}

impl Default for SpamApp
{
    fn default() -> Self
    {
        Self {
            //resize: 1,
            template_width: 8,
            template_height: 8,
            aspect_ratio: 1.0,
            use_aspect: true,
            template_path: String::from("template.txt"),
            template_success: false,
        }
    }
}

impl epi::App for SpamApp {
    fn name(&self) -> &str {
        "spam"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        frame.set_window_size(egui::Vec2{x: 512.0, y: 256.0});

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Template");

            ui.horizontal(|ui| {
                ui.label("width");
                ui.add(egui::DragValue::new(&mut self.template_width));
                ui.label("height");
                ui.add(egui::DragValue::new(&mut self.template_height));
                if self.use_aspect
                {
                    ui.label("aspect ratio");
                    ui.add(egui::DragValue::new(&mut self.aspect_ratio));
                    self.template_height = (self.aspect_ratio * self.template_width as f32) as u32;
                }
            });
            ui.checkbox(&mut self.use_aspect, "Use aspect ratio");

            ui.horizontal(|ui|{
                ui.label("Path to file");
                ui.text_edit_singleline(&mut self.template_path); 
            });

            if ui.button("Create template").clicked()
            {
                self.template_success = spam::create_template(&self.template_path, self.template_width, self.template_height);
            }
            
            //TODO : template_success = false when any value above is changed
            if self.template_success
            {
                ui.label("Template successfuly exported.");
            }
        });
    }
}

fn run_gui()
{
    println!("gui is running...");
    let app = SpamApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options); 
}
fn main() {
    run_gui();
}
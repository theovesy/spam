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

    process_path: String,
    process_resize: u32,
    output_path: String,
    output_success: bool,
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

            process_path: String::new(),
            process_resize: 10,
            output_path: String::from("output"),
            output_success: false,
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
            ui.collapsing("Template", |ui| {
                ui.horizontal(|ui| {
                    ui.label("width");
                    ui.add(egui::DragValue::new(&mut self.template_width)).on_hover_text("Template width in pixels");
                    ui.label("height");
                    ui.add(egui::DragValue::new(&mut self.template_height)).on_hover_text("Template height in pixels");
                    if self.use_aspect
                    {
                        ui.label("aspect ratio");
                        ui.add(egui::DragValue::new(&mut self.aspect_ratio));
                        self.template_height = (self.aspect_ratio * self.template_width as f32) as u32;
                    }
                });
                ui.checkbox(&mut self.use_aspect, "Use aspect ratio").on_hover_text("Compute height from width with a required aspect ratio");

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

            ui.collapsing("Process file", |ui| {
                ui.horizontal(|ui|{
                    ui.label("Path to file to process");
                    ui.text_edit_singleline(&mut self.process_path); 
                });

                ui.horizontal(|ui| {
                    ui.label("Resize factor");
                    ui.add(egui::DragValue::new(&mut self.process_resize)).on_hover_text("Factor by which the output file is to be resized");
                });

                ui.horizontal(|ui|{
                    ui.label("Path to outputfile (without extension)");
                    ui.text_edit_singleline(&mut self.output_path); 
                });

                if ui.button("Process file").clicked()
                {
                    self.output_success = spam::process_file(&self.process_path, &self.output_path, self.process_resize);
                }
                
                //TODO : output_success = false when any value above is changed
                if self.output_success
                {
                    ui.label("File successfuly processed!");
                }
            });
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
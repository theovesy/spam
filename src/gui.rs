use eframe::{epi, egui};

struct SpamApp {}

impl Default for SpamApp
{
    fn default() -> Self
    {
        Self {
        }
    }
}

impl epi::App for SpamApp {
   fn name(&self) -> &str {
       "spam"
   }

   fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("spam");
       });
   }
}

pub fn run_gui()
{
    println!("gui is running...");
    let app = SpamApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options); 
}
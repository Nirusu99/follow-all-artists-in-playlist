use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

const APP_NAME : &'static str = "FllpSpotify";
struct App {
    spotify: Option<Arc<Mutex<AuthCodeSpotify>>>,
}

pub fn run() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

impl App {
    fn login(&mut self, ui) {
        // TODO: program login
    }

    fn show_playlist_selection(&mut self, ui) {

    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Follow all artists in a given playlist");
            match self.spotify {
                Some(spotify) => self.show_playlist_selection(ui),
                None => self.login(ui)
            }
            ui.horizontal(|ui| {
                let name_label = ui.label("Username: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
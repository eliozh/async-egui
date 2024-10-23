use eframe::egui;
use eframe::egui::Ui;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);
    let _ = eframe::run_native(
        "egui with tokio",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::new(rx, tx)))),
    );
}

struct App {
    rx: Receiver<()>,
    tx: Sender<()>,
    value: u32,
    clicked: bool,
}

impl App {
    fn new(rx: Receiver<()>, tx: Sender<()>) -> Self {
        Self {
            rx,
            tx,
            value: 1,
            clicked: false,
        }
    }
}

fn create_button(ui: &mut Ui, app: &mut App, enabled: bool) {
    let button = egui::widgets::Button::new("inc");
    if ui.add_enabled(enabled, button).clicked() {
        let tx = app.tx.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(3)).await;
            tx.send(()).await.expect("Failed to send data");
        });
        app.clicked = true;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.value, 1..=100).text("value"));

            match self.rx.try_recv() {
                Err(_) if self.clicked == true => {
                    create_button(ui, self, false);
                }
                Err(_) if self.clicked == false => {
                    create_button(ui, self, true);
                }
                Ok(_) => {
                    create_button(ui, self, true);
                    self.value += 1;
                    self.clicked = false;
                }
                _ => {}
            }
        });
    }
}

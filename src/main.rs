
use chrono::Local;
use eframe::egui;
use std::fs::OpenOptions;
use std::io::Write;

struct KeyboardApp {
    running: bool,
    events: Vec<String>,
    log: Option<std::fs::File>,
    message: String,
}

impl Default for KeyboardApp {
    fn default() -> Self {
        Self {
            running: false,
            events: Vec::new(),
            log: None,
            message: "Нажми «Старт», чтобы начать тест".to_string(),
        }
    }
}

impl KeyboardApp {
    fn start(&mut self) {
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("keyboard_log.txt")
        {
            Ok(file) => {
                self.log = Some(file);
                self.running = true;
                self.message = "Запись событий включена".to_string();
            }
            Err(error) => {
                self.message = format!("Ошибка открытия журнала: {error}");
            }
        }
    }

    fn stop(&mut self) {
        self.running = false;
        if let Some(mut file) = self.log.take() {
            let _ = file.flush();
        }
        self.message = "Запись остановлена".to_string();
    }

    fn record(&mut self, key: String) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let line = format!("{timestamp} | {key}");

        self.events.push(line.clone());

        if self.events.len() > 100 {
            self.events.remove(0);
        }

        if let Some(file) = &mut self.log {
            let _ = writeln!(file, "{line}");
            let _ = file.flush();
        }
    }
}

impl eframe::App for KeyboardApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let events = ui.input(|input| input.events.clone());

        ui.heading("Keyboard Interceptor");
        ui.label("Тестирование клавиатуры внутри этого окна.");

        ui.horizontal(|ui| {
            if ui.button("Старт").clicked() {
                self.start();
            }

            if ui.button("Стоп").clicked() {
                self.stop();
            }
        });

        ui.label(&self.message);
        ui.separator();

        ui.heading("Тестовая область");
        ui.label("Щёлкни здесь и нажимай клавиши.");

        let response = ui.allocate_response(
            egui::vec2(ui.available_width(), 120.0),
            egui::Sense::click(),
        );

        ui.painter().rect_filled(
            response.rect,
            5.0,
            ui.visuals().faint_bg_color,
        );

        ui.painter().text(
            response.rect.center(),
            egui::Align2::CENTER_CENTER,
            "Область тестирования клавиш",
            egui::FontId::proportional(16.0),
            ui.visuals().text_color(),
        );

        if response.clicked() {
            response.request_focus();
        }

        if self.running && response.has_focus() {
            for event in events {
                if let egui::Event::Key {
                    key,
                    pressed,
                    repeat,
                    ..
                } = event
                {
                    let kind = if pressed { "Нажатие" } else { "Отпускание" };

                    self.record(format!(
                        "Клавиша: {key:?} | Событие: {kind} | Повтор: {repeat}"
                    ));
                }
            }
        }

        ui.separator();
        ui.heading("Последние события");

        egui::ScrollArea::vertical()
            .max_height(220.0)
            .show(ui, |ui| {
                for event in self.events.iter().rev() {
                    ui.label(event);
                }
            });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Keyboard Interceptor",
        options,
        Box::new(|_cc| Ok(Box::new(KeyboardApp::default()))),
    )
}

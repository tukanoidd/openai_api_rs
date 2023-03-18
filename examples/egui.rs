use eframe::{Frame, NativeOptions, Theme};
use egui::{CentralPanel, Context, ScrollArea, TextEdit, Vec2, Widget};
use miette::IntoDiagnostic;
use once_cell::sync::Lazy;

use openai_api_rs::{client::Client, model::Model};

static CLIENT: Lazy<Client> =
    Lazy::new(|| Client::new(dotenvy::var("OPENAI_API_KEY").into_diagnostic().unwrap()));

fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let options = NativeOptions {
        icon_data: None,
        min_window_size: Some(Vec2::new(800.0, 600.0)),
        resizable: true,
        default_theme: Theme::Dark,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "egui_example",
        options,
        Box::new(|_cc| Box::new(App::new(&CLIENT))),
    )
    .map_err(|e| miette::miette!("Failed to run the egui example: {}", e))?;

    Ok(())
}

struct App<'client> {
    #[allow(dead_code)]
    client: &'client Client,
    text_davinci_model: Model<'client>,

    text: String,
    result_text: String,
}

impl<'client> App<'client> {
    fn new(client: &'client Client) -> Self {
        let text_davinci_model = client
            .retrieve_model_info_blocking("text-davinci-003")
            .expect("Failed to retrieve text-davinci-003 model");

        Self {
            client,
            text_davinci_model,

            text: String::new(),
            result_text: String::new(),
        }
    }
}

impl<'client> eframe::App for App<'client> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ScrollArea::vertical()
                    .id_source("text")
                    .max_height(ui.available_height())
                    .max_width(ui.available_width() * 0.45)
                    .show(ui, |ui| {
                        TextEdit::multiline(&mut self.text)
                            .min_size(ui.available_size())
                            .ui(ui);
                    });

                ui.add_space(5.0);

                if ui.button("Create a completion ->").clicked() {
                    let request = self
                        .text_davinci_model
                        .init_completion_request_builder()
                        .add_prompt(&self.text);
                    let completion = self
                        .text_davinci_model
                        .request_completion_blocking(request)
                        .expect("Failed to create completion");

                    self.result_text = completion
                        .choices
                        .iter()
                        .map(|c| format!("{}{}", self.text, c.text))
                        .collect::<Vec<_>>()
                        .join("\n|---------------------------------------------------|\n");
                }

                ui.add_space(5.0);

                ScrollArea::vertical()
                    .id_source("result")
                    .max_height(ui.available_height())
                    .max_width(ui.available_width())
                    .show(ui, |ui| {
                        TextEdit::multiline(&mut self.result_text)
                            .min_size(ui.available_size())
                            .ui(ui);
                    });
            });
        });
    }
}

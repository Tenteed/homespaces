use iced::widget::{Column, Scrollable, Text};
use iced::{Sandbox, Settings};
use std::collections::VecDeque;

mod tools;
#[derive(Default)]
struct AppViewer {
    applications: VecDeque<tools::Application>,
}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for AppViewer {
    type Message = Message;

    fn new() -> Self {
        let applications = tools::get_installed_applications();
        Self { applications }
    }

    fn title(&self) -> String {
        "Application Viewer".into()
    }

    // TODO Handle user input for application selection
    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = self
            .applications
            .iter()
            .fold(Column::new().spacing(0), |column, app| {
                column
                    .push(Text::new(format!("Name: {}", app.name)))
            });
        Scrollable::new(content).into()
    }
}

fn main() -> iced::Result {
    AppViewer::run(Settings::default())
}

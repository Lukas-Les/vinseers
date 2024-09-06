mod constants;
mod helpers;

use std::string::ToString;

use rfd::FileDialog;

use iced::widget::text_editor::Content;
use iced::widget::{text_editor, Button, Column, Container, Row};
use iced::{Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Vinseers::run(Settings::default())
}

struct Vinseers {
    content: text_editor::Content,
    vin_re: String,
}

impl Sandbox for Vinseers {
    type Message = Message;

    fn new() -> Vinseers {
        Vinseers {
            content: text_editor::Content::with_text(constants::RESULT_TEXT_DEFAULT),
            vin_re: constants::VIN_RE_DEFAULT.to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("vinseers")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SelectFiles => {
                let target_paths = FileDialog::new()
                    .add_filter("text", &constants::ALLOWED_FILES)
                    .pick_files();
                if let Some(paths) = target_paths {
                    let result = helpers::process_paths(&paths, &self.vin_re);
                    self.content = Content::with_text(result.join("\n").as_str());
                }
            }
            Message::SelectDir => {
                let target_paths = FileDialog::new().pick_folders();
                if let Some(paths) = target_paths {
                    let result = helpers::process_paths(&paths, &self.vin_re);
                    self.content = Content::with_text(result.join("\n").as_str());
                }
            }
            Message::ResetResult => {
                self.content = text_editor::Content::with_text(constants::RESULT_TEXT_DEFAULT);
            }
            Message::AnyActionPerformed(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let top_row = Row::new()
            .push(Button::new("Select Files").on_press(Message::SelectFiles))
            .push(Button::new("Select directory").on_press(Message::SelectDir))
            .push(Button::new("Reset").on_press(Message::ResetResult))
            .spacing(3);

        let top_row_container = Container::new(top_row).padding(3).center_x();
        let result_display = text_editor(&self.content).on_action(Message::AnyActionPerformed);
        let content = Column::new().push(top_row_container).push(result_display);
        content.into()
    }
}

#[derive(Clone, Debug)]
enum Message {
    SelectFiles,
    SelectDir,
    ResetResult,
    AnyActionPerformed(text_editor::Action),
}

#[derive(Clone, Debug, PartialEq)]
enum VidType {
    Vin,
    Lpn,
}

impl VidType {
    const ALL: &'static [Self] = &[Self::Vin, Self::Lpn];
}

impl ToString for VidType {
    fn to_string(&self) -> String {
        match self {
            VidType::Vin => {
                return "Vin".to_string();
            }
            VidType::Lpn => return "Lpn".to_string(),
        }
    }
}

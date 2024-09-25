mod constants;
mod helpers;

use std::string::ToString;

use rfd::FileDialog;

use iced::widget::text_editor::Content;
use iced::widget::{row, text_editor, Button, column};
use iced::{Element, Theme};

pub fn main() -> iced::Result {
    iced::application("vinseers", update, view)
        .theme(theme)
        .run()
}

fn theme(state: &State) -> Theme {
    Theme::Dark
}


struct State {
    content: text_editor::Content,
    current_regex: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            content: text_editor::Content::with_text(constants::RESULT_TEXT_DEFAULT),
            current_regex: constants::VIN_RE_DEFAULT.to_string(),
        }
    }
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::SelectFiles => {
            let target_paths = FileDialog::new()
                .add_filter("text", &constants::ALLOWED_FILES)
                .pick_files();
            if let Some(paths) = target_paths {
                let result = helpers::process_paths(&paths, &state.current_regex);
                state.content = Content::with_text(result.join("\n").as_str());
            }
        }
        Message::SelectDir => {
            let target_paths = FileDialog::new().pick_folders();
            if let Some(paths) = target_paths {
                let result = helpers::process_paths(&paths, &state.current_regex);
                state.content = Content::with_text(result.join("\n").as_str());
            }
        }
        Message::ResetResult => {
            state.content = text_editor::Content::with_text(constants::RESULT_TEXT_DEFAULT);
        }
        Message::AnyActionPerformed(action) => {
            state.content.perform(action);
        }
    }
}

fn view(state: &State) -> Element<Message> {
    let top_row = row![
        Button::new("Select Files").on_press(Message::SelectFiles),
        Button::new("Select directory").on_press(Message::SelectDir),
        Button::new("Reset").on_press(Message::ResetResult),
    ]
        .spacing(3);
    let result_display = text_editor(&state.content).on_action(Message::AnyActionPerformed);

    column![
        top_row,
        result_display,
    ]
        .spacing(3)
        .into()
}


#[derive(Clone, Debug)]
enum Message {
    SelectFiles,
    SelectDir,
    ResetResult,
    AnyActionPerformed(text_editor::Action),
}


#[derive(Clone, Debug, PartialEq)]
enum LpnType {
    Ltu,
}


#[derive(Clone, Debug, PartialEq)]
enum VidType {
    Vin,
    Lpn(LpnType),
}

impl VidType {
    fn to_regex(&self) -> String {
        match self {
            Self::Vin => constants::VIN_RE_DEFAULT.to_string(),
            Self::Lpn(t) => {
                match t {
                    LpnType::Ltu => constants::LPN_LTU_RE.to_string(),
                }
            }
        }
    }
}
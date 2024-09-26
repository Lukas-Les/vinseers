mod constants;

use std::string::ToString;

use rfd::FileDialog;

use iced::widget::text_editor::Content;
use iced::widget::{row, text_editor, Button, column, PickList};
use iced::{Element, Theme};

use vinseers::{helpers, regexes};


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
    vid: VidType,
}

impl Default for State {
    fn default() -> Self {
        Self {
            content: text_editor::Content::with_text(constants::RESULT_TEXT_DEFAULT),
            vid: VidType::Vin,
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
                let result = helpers::process_paths(&paths, &state.vid.to_regex());
                state.content = Content::with_text(result.join("\n").as_str());
            }
        }
        Message::SelectDir => {
            let target_paths = FileDialog::new().pick_folders();
            if let Some(paths) = target_paths {
                let result = helpers::process_paths(&paths, &state.vid.to_regex());
                state.content = Content::with_text(result.join("\n").as_str());
            }
        }
        Message::ResetResult => {
            state.content = text_editor::Content::with_text(constants::RESULT_TEXT_DEFAULT);
        }
        Message::AnyActionPerformed(action) => {
            state.content.perform(action);
        },
        Message::VidSelected(vid) => {
            state.vid = vid;
            println!("vid regex: {}", &state.vid.to_regex());
        },
    }
}

fn view(state: &State) -> Element<Message> {
    let re_pick_list = PickList::new(VidType::ALL.as_ref(), Some(&state.vid), Message::VidSelected);

    let top_row = row![
        Button::new("Select Files").on_press(Message::SelectFiles),
        Button::new("Select directory").on_press(Message::SelectDir),
        Button::new("Reset").on_press(Message::ResetResult),
        re_pick_list,
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
    VidSelected(VidType),
    AnyActionPerformed(text_editor::Action),
}


#[derive(Clone, Debug, PartialEq, Eq)]
enum LpnType {
    Ltu,
}


#[derive(Clone, Debug, PartialEq, Eq)]
enum VidType {
    Vin,
    Lpn(LpnType),
}



impl VidType {
    const ALL: &'static [Self; 2] = &[Self::Vin, Self::Lpn(LpnType::Ltu)];

    fn to_regex(&self) -> String {
        match self {
            Self::Vin => regexes::VIN_DEFAULT.to_string(),
            Self::Lpn(t) => {
                match t {
                    LpnType::Ltu => regexes::LPN_LTU.to_string(),
                }
            }
        }
    }
}

impl ToString for VidType {
    fn to_string(&self) -> String {
        match self {
            VidType::Vin => "VIN".to_string(),
            VidType::Lpn(t) => {
                match t {
                    LpnType::Ltu => "LPN-LTU".to_string(),
                }
            }
        }
    }
}

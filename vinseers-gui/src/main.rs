mod constants;

use std::fs::{File, read_dir};
use std::io::prelude::*;

use druid::widget::{Button, Flex, Scroll, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use druid::commands::{SHOW_OPEN_PANEL, OPEN_FILE, OPEN_FILES};
use druid::{FileDialogOptions, FileSpec, Target, Selector, Command};

use vinseers::{outputs, search};

#[derive(Clone, Data, Lens)]
struct AppState {
    result: String,
    re_pattern: String,
}

const UPDATE_CONTENT: Selector<String> = Selector::new("update-content");
const OPEN_FILE_PATH: Selector<Vec<std::path::PathBuf>> = Selector::new("druid-builtin.open-file-path");
const OPEN_FILES_PATH: Selector<Vec<std::path::PathBuf>> = Selector::new("druid-builtin.open-files-path");

fn build_ui() -> impl Widget<AppState> {
    let select_files_button = Button::new("Select files")
        .on_click(|ctx, _data: &mut AppState, _env| {
            println!("files button clicked");
            let options = FileDialogOptions::new()
                .allowed_types(vec![FileSpec::new("Text files", &constants::ALLOWED_FILE_TYPES)])
                .default_type(FileSpec::new("Text file", &["txt"]))
                .multi_selection()
                .name_label("Select files")
                .title("Choose files to scan")
                .button_text("Scan");
            ctx.submit_command(SHOW_OPEN_PANEL.with(options).to(Target::Auto));
        });
    
    let select_dirs_button = Button::new("Select directories")
        .on_click(|ctx, _data: &mut AppState, _env| {
            println!("dir button clicked");
            let options = FileDialogOptions::new()
                .select_directories()
                .multi_selection()
                .name_label("Select directories")
                .title("Choose directories to open")
                .button_text("Scan");
            ctx.submit_command(SHOW_OPEN_PANEL.with(options).to(Target::Auto));
        });

    let file_content = TextBox::multiline()
        .with_placeholder("Results will be displayed here")
        .lens(AppState::result);

    let result_label = Scroll::new(file_content).vertical();
    
    let buttons_row = Flex::row()
        .with_child(select_files_button)
        .with_spacer(8.0)
        .with_child(select_dirs_button);

    Flex::column()
        .with_child(buttons_row)
        .with_spacer(8.0)
        .with_child(result_label)
}

fn main() -> Result<(), druid::PlatformError> {
    println!("hello");
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("vinseers").with_placeholder("Search VIN"))
        .window_size((1000.0, 400.0));

    AppLauncher::with_window(main_window)
        .delegate(AppDelegate)
        .launch(AppState {
            result: "".to_string(),
            re_pattern: constants::DEFAULT_RE_PATTERN.to_string(),
        })
        .expect("Failed to launch application");

    Ok(())
}

struct AppDelegate;

impl druid::AppDelegate<AppState> for AppDelegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &Env,
    ) -> druid::Handled {
        println!("Received command: {:?}", cmd);

        // Check for OPEN_FILES command
        if let Some(file_infos) = cmd.get(OPEN_FILES) {
            println!("OPEN_FILES command received");
            for file_info in file_infos {
                if let Some(path) = file_info.path().to_str() {
                    process_file_info(ctx, path, data);
                }
            }
            return druid::Handled::Yes;
        }

        // Check for OPEN_FILE command
        if let Some(file_info) = cmd.get(OPEN_FILE) {
            println!("OPEN_FILE command received");
            if let Some(path) = file_info.path().to_str() {
                process_file_info(ctx, path, data);
            }
            return druid::Handled::Yes;
        }

        // Check for druid-builtin.open-file-path command
        if let Some(file_paths) = cmd.get(OPEN_FILE_PATH) {
            println!("druid-builtin.open-file-path command received");
            for file_path in file_paths {
                if let Some(path_str) = file_path.to_str() {
                    process_file_info(ctx, path_str, data);
                }
            }
            return druid::Handled::Yes;
        }

        // Check for druid-builtin.open-files-path command
        if let Some(file_paths) = cmd.get(OPEN_FILES_PATH) {
            println!("druid-builtin.open-files-path command received");
            for file_path in file_paths {
                if let Some(path_str) = file_path.to_str() {
                    process_file_info(ctx, path_str, data);
                }
            }
            return druid::Handled::Yes;
        }

        if let Some(new_content) = cmd.get(UPDATE_CONTENT) {
            println!("Updating content");
            data.result = format!("{}\n{}", data.result, new_content);
            return druid::Handled::Yes;
        }

        druid::Handled::No
    }
}

fn process_file_info(ctx: &mut druid::DelegateCtx, path: &str, data: &mut AppState) {
    let results = process_path_recursive(path, &data.re_pattern);
    let results_str = results.join("\n");
    ctx.submit_command(Command::new(UPDATE_CONTENT, results_str, Target::Auto));
}

fn process_path_recursive(path: &str, re_pattern: &str) -> Vec<String> {
    let mut results = Vec::new();
    let path = std::path::Path::new(path);
    if path.is_dir() {
        println!("Processing directory: {}", path.display());
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(entry_path) = entry.path().to_str() {
                        results.extend(process_path_recursive(entry_path, re_pattern));
                    }
                }
            }
        }
    } else {
        println!("Processing file: {}", path.display());
        if let Ok(mut file) = File::open(path) {
            let mut buffer = String::new();
            if file.read_to_string(&mut buffer).is_ok() {
                let path_string = path.to_str().unwrap().to_string();
                let result = outputs::format(&path_string, search::search(&buffer, &re_pattern.to_string()));
                results.push(result);
            } else {
                println!("Failed to read file: {}", path.display());
            }
        } else {
            println!("Failed to open file: {}", path.display());
        }
    }
    results
}

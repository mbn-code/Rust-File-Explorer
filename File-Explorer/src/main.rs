use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Selector, SingleUse, Target, Widget, WidgetExt, WindowDesc};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use std::env;



#[derive(Clone, Data, Lens)]
struct AppState {
    #[lens(ignore)]
    root_path: Arc<Mutex<PathBuf>>,
    search_term: String,
    result: String,
}

fn build_ui() -> impl Widget<AppState> {
    druid::widget::Flex::column()
        .with_child(druid::widget::TextBox::new().lens(AppState::search_term))
        .with_child(druid::widget::Button::new("Search").on_click(|_, data: &mut AppState, _| {
            let root_path = data.root_path.lock().unwrap().clone();
            let search_term = data.search_term.clone();

            let result = search_files(&root_path, &search_term);
            data.result = result;
        }))
        .with_child(druid::widget::Label::new(|data: &AppState, _env: &_| data.result.clone()))
}

fn search_files(root_path: &Path, search_term: &str) -> String {
    let mut result = String::new();

    for entry in WalkDir::new(root_path) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().contains(search_term) {
                        result.push_str(&format!("{}\n", path.display()));
                    }
                }
            }
        }
    }

    result
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
    .title(LocalizedString::new("File Explorer"));

    let root_path = Arc::new(Mutex::new(env::current_dir().unwrap()));

    let app_state = AppState {
        root_path: root_path.clone(),
        search_term: String::new(),
        result: String::new(),
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(app_state)
        .expect("Failed to launch application");
}
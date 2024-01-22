use druid::{widget, AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use regex::Regex;

use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use walkdir::WalkDir;

#[derive(Clone, Data, Lens)]
struct AppState {
    #[lens(ignore)]
    root_path: Arc<Mutex<PathBuf>>,
    search_term: String,
    result: String,
}

#[derive(Clone, Data, Lens)]
struct SearchUpdate {
    result: String,
}

fn build_ui() -> impl Widget<AppState> {
    let label = widget::Label::new(|data: &AppState, _env: &_| data.result.clone()).with_text_size(20.0);

    let scrollable_label = druid::widget::Scroll::new(label);

    widget::Flex::column()
        .with_child(widget::TextBox::new().lens(AppState::search_term))
        .with_child(
            widget::Button::new("Search").on_click(|_, data: &mut AppState, _| {
                let root_path = data.root_path.lock().unwrap().clone();
                let search_term = data.search_term.clone();

                let (tx, rx) = mpsc::channel();
                let tx_copy = tx.clone();

                thread::spawn(move || {
                    let result = search_files(&root_path, &search_term, Some(tx));
                    let _ = tx_copy.send(SearchUpdate { result });
                });

                let mut result = String::new();
                for update in rx.iter() {
                    result.push_str(&update.result);
                    data.result = result.clone();
                }
            }),
        )
        .with_child(scrollable_label)
}

// This function takes in a root path and a search term and returns a string of the search results
fn search_files(root_path: &Path, search_term: &str, tx: Option<mpsc::Sender<SearchUpdate>>) -> String {
    
    let mut result = String::new();
    let search_term_regex = Regex::new(&format!(r"(?i){}", search_term)).expect("Invalid regex");

    WalkDir::new(root_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| search_term_regex.is_match(name))
                .unwrap_or(false)
        })
        .for_each(|entry| {
            let found_path = format!("{}\n", entry.path().display());
            result.push_str(&found_path);
            if let Some(tx) = &tx {
                let _ = tx.send(SearchUpdate {
                    result: found_path.clone(),
                });
            }
        });

    result
}


fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("File Explorer"));

    let root_path = Arc::new(Mutex::new(std::env::current_dir().unwrap()));

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

use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::widget::prelude::*;
use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Lens, Selector, Target,
    Widget, WidgetExt, WindowDesc,
};
use regex::Regex;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use walkdir::WalkDir;

// A selector for updating search results from a background thread.
// Note: Now the payload is an Arc<Vec<String>>
const UPDATE_SEARCH_RESULTS: Selector<Arc<Vec<String>>> =
    Selector::new("update_search_results");

#[derive(Clone, Data, Lens)]
struct AppState {
    pub root_path: String,
    pub search_term: String,
    // Change from im::Vector<String> to Arc<Vec<String>> for compatibility with ListIter
    pub search_results: Arc<Vec<String>>,
}

fn build_ui() -> impl Widget<AppState> {
    // Button to let the user choose a directory (using rfd for a native dialog)
    let choose_dir_btn = Button::new("Choose Directory").on_click(|_ctx, data: &mut AppState, _env| {
        // Use rfd's file dialog (this will show a native folder chooser on macOS)
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            data.root_path = path.to_string_lossy().to_string();
            // Clear any previous search results when the directory changes.
            data.search_results = Arc::new(Vec::new());
        }
    });

    // A label showing the currently selected directory.
    let dir_label = Label::new(|data: &AppState, _env: &_| {
        format!("Current Directory: {}", data.root_path)
    })
    .with_text_size(14.0);

    // Text box for entering the search term.
    let search_box = TextBox::new()
        .with_placeholder("Enter search term")
        .lens(AppState::search_term);

    // Button to kick off the search.
    let search_btn = Button::new("Search").on_click(|ctx, data: &mut AppState, _env| {
        let root = data.root_path.clone();
        let term = data.search_term.clone();

        // Clear any previous search results.
        data.search_results = Arc::new(Vec::new());

        let sink = ctx.get_external_handle();

        thread::spawn(move || {
            let results = search_files(&root, &term);
            // Send the search results back to the UI thread.
            sink.submit_command(UPDATE_SEARCH_RESULTS, results, Target::Auto)
                .expect("Failed to submit command");
        });
    });

    // Create a list widget to display search results.
    let results_list = List::new(|| {
        Label::new(|item: &String, _env: &_| format!("{}", item))
            .padding(5.0)
    })
    .with_spacing(2.0)
    // Lens into the search_results field (which is now an Arc<Vec<String>>)
    .lens(AppState::search_results);

    // Layout the UI elements vertically.
    Flex::column()
        .with_child(choose_dir_btn.padding(5.0))
        .with_child(dir_label.padding(5.0))
        .with_child(search_box.padding(5.0))
        .with_child(search_btn.padding(5.0))
        .with_flex_child(Scroll::new(results_list), 1.0)
}

/// Searches files under the given directory whose names match the search term (case-insensitive)
/// and returns an Arc<Vec<String>>.
fn search_files(root_path: &str, search_term: &str) -> Arc<Vec<String>> {
    let regex = Regex::new(&format!(r"(?i){}", search_term)).unwrap();
    let root = PathBuf::from(root_path);
    let mut results = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                if regex.is_match(name) {
                    results.push(entry.path().display().to_string());
                }
            }
        }
    }
    Arc::new(results)
}

/// A delegate to handle commands coming from the background thread.
struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> druid::Handled {
        if let Some(results) = cmd.get(UPDATE_SEARCH_RESULTS) {
            data.search_results = results.clone();
            return druid::Handled::Yes;
        }
        druid::Handled::No
    }
}

fn main() {
    // Create the main window.
    let main_window = WindowDesc::new(build_ui()).title("macOS File Explorer");

    // Initialize the state with the current directory.
    let initial_state = AppState {
        root_path: std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .display()
            .to_string(),
        search_term: "".to_string(),
        search_results: Arc::new(Vec::new()),
    };

    // Launch the application with the delegate to handle background commands.
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}

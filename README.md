# Rust File Explorer
A simple file explorer written in Rust using the Druid framework.
This file explorer utilises regex in Rust and works on both Windows and macOS

## Features

- Search for files by name in the specified directory.
- Displays search results with a case-insensitive regex match for file names.
- User interface powered by the Druid framework.
- Results are shown in a scrollable window.

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/CollinEdward/Rust-Windows-File-Explorer.git
   ```

2. Navigate to the project directory:

   ```bash
   cd Rust-Windows-File-Explorer
   ```

3. Build and run the application:

   ```bash
   cargo run
   ```

4. Enter the directory path and search term in the provided text boxes.
5. Click the "Search" button to initiate the search.

## Dependencies

- [Druid](https://github.com/linebender/druid): A data-driven Rust GUI framework.
- [walkdir](https://crates.io/crates/walkdir): A simple filesystem walker.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

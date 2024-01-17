/*!

 [![github]](https://github.com/YM162/gulagcleaner)

  [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github

 # GulagCleaner crate in Rust
 Gulag Cleaner is a tool designed to remove advertisements from PDFs, making it easier to read and navigate documents without being disrupted by unwanted ads.

 # Examples

    ```rust, ignore
    use gulagcleaner_rs::clean::clean_pdf;
    use std::fs;
    use walkdir::WalkDir;

    fn main() {
        let root_folder = std::env::var("HOME").unwrap_or(".".to_string());
        let substring = "wuolah";
        let replacement = "clean";

        for entry in WalkDir::new(root_folder).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                if let Some(file_name) = file_path.file_name() {
                    if file_name.to_string_lossy().contains(substring)
                        && file_path.extension().map_or(false, |ext| ext == "pdf")
                    {
                        println!("Cleaning {}", file_name.to_str().unwrap());
                        let data = fs::read(file_path).unwrap();
                        let (clean_pdf, _) = clean_pdf(data, false);
                        let output_path = file_path.with_file_name(
                            file_name.to_string_lossy().replace(substring, replacement),
                        );
                        fs::create_dir_all(output_path.parent().unwrap()).unwrap(); // Create the output directory if it doesn't exist
                        println!(
                            "\tClean version output to -> {}",
                            &output_path.to_str().unwrap()
                        );
                        fs::write(output_path, clean_pdf).unwrap();
                    }
                }
            }
        }
    }

    ```
*/
/// Main method execution
pub mod clean;

/// Main method rexport
pub use clean::clean_pdf;

/// Modeling the different pdf sources and types
pub mod models {
    /// Represents the different methods used in the Gulag Cleaner application.
    pub mod method;

    /// Represents the different page types used in the Gulag Cleaner application.
    pub mod page_type;
}

#[cfg(test)]
pub mod tests;

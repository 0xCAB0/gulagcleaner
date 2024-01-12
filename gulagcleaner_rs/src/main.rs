use gulagcleaner_rs::clean::clean_pdf;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let root_folder = std::env::var("HOME").unwrap() + "/UPM";
    let substring = "wuolah";
    let replacement = "clean";

    for entry in WalkDir::new(&root_folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if let Some(file_name) = file_path.file_name() {
                if file_name.to_string_lossy().contains(substring)
                    && file_path.extension().map_or(false, |ext| ext == "pdf")
                {
                    println!("Cleaning {}", file_path.to_str().unwrap());
                    let data = fs::read(file_path).unwrap();
                    let (clean_pdf, _) = clean_pdf(data, false);
                    let output_path = file_path.with_file_name(
                        file_name.to_string_lossy().replace(substring, replacement),
                    );
                    fs::create_dir_all(output_path.parent().unwrap()).unwrap(); // Create the output directory if it doesn't exist
                    println!("Clean version output to {}", &output_path.to_str().unwrap());
                    fs::write(output_path, clean_pdf).unwrap();
                }
            }
        }
    }
}

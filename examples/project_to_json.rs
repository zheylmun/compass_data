use std::path::PathBuf;

use compass_data::Project;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <project_file>", args[0]);
        std::process::exit(1);
    }
    let project_path = PathBuf::from(&args[1]);
    if !project_path.exists() {
        eprintln!("Project file not found: {}", args[1]);
        std::process::exit(1);
    }
    let output_path = project_path.with_extension("json");

    let project = Project::read(&project_path).unwrap();
    let loaded = project.load_survey_files().unwrap();
    let json = serde_json::to_string_pretty(&loaded).unwrap();
    std::fs::write(output_path, &json).unwrap();
}

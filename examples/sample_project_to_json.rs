use compass_data::Project;

fn main() {
    let project = Project::read("test_data/Fulfords.mak").unwrap();
    let loaded = project.load_survey_files().unwrap();
    let json = serde_json::to_string_pretty(&loaded).unwrap();
    println!("{}", json);
}

# Compass Data

[![Crates.io Version](https://img.shields.io/crates/v/compass_data?style=for-the-badge)](https://crates.io/crates/compass_data)
[![docs.rs](https://img.shields.io/docsrs/compass_data?style=for-the-badge)](https://docs.rs/compass_data/latest/compass_data/)
[![GitHub branch status](https://img.shields.io/github/checks-status/zheylmun/compass_data/main?style=for-the-badge&logo=GitHub)](https://github.com/zheylmun/compass_data/actions)
[![Codecov](https://img.shields.io/codecov/c/github/zheylmun/compass_data?style=for-the-badge&logo=CodeCov)](https://app.codecov.io/gh/zheylmun/compass_data)

A Rust library for reading, writing, and working with [Compass](https://www.fountainware.com/compass/index.htm)
cave survey project files (`.mak`) and survey data files (`.dat`).

## Overview

This library enables interoperation and data sharing between cave survey software by providing
a complete implementation of the Compass file formats. It supports:

- **Project files (`.mak`)**: Parse and create project files that organize multiple survey data files
- **Survey data files (`.dat`)**: Parse and serialize individual survey data with shots, stations, and metadata
- **Coordinate systems**: UTM coordinates with support for 23 geodetic datums
- **Rich error reporting**: Detailed error messages with source locations using [miette](https://docs.rs/miette)

## Usage

### Reading a Project File

```rust,no_run
use compass_data::Project;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a project file (parses metadata but doesn't load survey data yet)
    let project = Project::read(&PathBuf::from("cave.mak"))?;

    println!("Project has {} survey files", project.survey_files.len());

    // Access file metadata without loading survey data
    for dat_file in &project.survey_files {
        println!("  File: {}", dat_file.file_path.display());
        println!("  Datum: {:?}", dat_file.file_state.datum);
    }

    // Load all survey data from disk
    let loaded_project = project.load_survey_files()?;

    // Now access the actual survey data
    for dat_file in &loaded_project.survey_files {
        for survey in dat_file.surveys() {
            println!("Survey: {} - {} shots", survey.name, survey.shots.len());
        }
    }
    Ok(())
}
```

### Reading a Survey Data File Directly

```rust,no_run
use compass_data::Survey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("survey.dat")?;
    let surveys = Survey::parse_dat_file(&contents)?;

    for survey in &surveys {
        println!("Cave: {}", survey.cave_name);
        println!("Survey: {}", survey.name);
        println!("Date: {}", survey.date);
        println!("Shots: {}", survey.shots.len());
    }
    Ok(())
}
```

### Serializing Survey Data

```rust,no_run
use compass_data::Survey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse and re-serialize (round-trip)
    let contents = std::fs::read_to_string("survey.dat")?;
    let surveys = Survey::parse_dat_file(&contents)?;

    for survey in &surveys {
        let serialized = survey.serialize();
        std::fs::write(format!("{}.dat", survey.name), serialized)?;
    }
    Ok(())
}
```

## Key Types

| Type | Description |
|------|-------------|
| [`Project`] | A Compass project file containing references to survey data files |
| [`DatFile`] | A survey data file reference with its associated state |
| [`Survey`] | A single survey containing shots, team info, date, and parameters |
| [`Shot`] | A single survey shot with distance, azimuth, inclination, and LRUD |
| [`Datum`] | Geodetic datum (NAD27, NAD83, WGS84, etc.) |
| [`FileState`] | Rolling state captured when a file is encountered during parsing |

## Features

- **`serde`**: Enable serialization/deserialization with serde
- **`cli`**: Build the `compass` CLI tool for inspecting project and survey files

## CLI Tool

When built with the `cli` feature, a command-line tool is available:

```bash
# Install
cargo install compass_data --features cli

# Inspect a project file
compass inspect project cave.mak

# Inspect with full survey data
compass inspect project cave.mak --load

# Inspect a survey data file
compass inspect survey cave.dat

# Show detailed shot data
compass inspect survey cave.dat --verbose
```

## File Format Documentation

The Compass file formats are documented at:
- [Project File Format](https://www.fountainware.com/compass/HTML_Help/Project_Manager/projectfileformat.htm)
- [Survey Data File Format](https://www.fountainware.com/compass/HTML_Help/Surveying/surveyfileformat.htm)

## License

Licensed under either:
- [Apache License Version 2.0](./LICENSE-APACHE), or
- [MIT license](./LICENSE-MIT)

at your option.

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.

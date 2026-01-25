//! Compass CLI - Inspect cave survey project and data files

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use compass_data::{Project, Survey};

#[derive(Parser)]
#[command(name = "compass")]
#[command(about = "Inspect Compass cave survey files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inspect cave survey files
    Inspect {
        #[command(subcommand)]
        target: InspectTarget,
    },
}

#[derive(Subcommand)]
enum InspectTarget {
    /// Inspect a project file (.mak)
    Project {
        /// Path to the project file
        path: PathBuf,
        /// Load all survey data files
        #[arg(short, long)]
        load: bool,
    },
    /// Inspect a survey data file (.dat)
    Survey {
        /// Path to the survey data file
        path: PathBuf,
        /// Show detailed shot data
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { target } => match target {
            InspectTarget::Project { path, load } => inspect_project(&path, load),
            InspectTarget::Survey { path, verbose } => inspect_survey(&path, verbose),
        },
    }
}

fn inspect_project(path: &PathBuf, load: bool) -> miette::Result<()> {
    let project = Project::read(path)?;

    println!("Project: {}", path.display());
    if let Some(id) = &project.id {
        println!("  ID: {id}");
    }
    println!("  Survey files: {}", project.survey_files.len());
    println!();

    if load {
        let loaded = project.load_survey_files()?;
        for dat_file in &loaded.survey_files {
            println!("  File: {}", dat_file.file_path.display());
            println!("    Datum: {:?}", dat_file.file_state.datum);
            let loc = &dat_file.file_state.base_location;
            println!(
                "    Base location: UTM Zone {}, E:{:.3} N:{:.3} Elev:{:.3}m",
                loc.zone,
                loc.east_north_elevation.easting,
                loc.east_north_elevation.northing,
                loc.east_north_elevation.up
            );
            println!("    Fixed stations: {}", dat_file.project_stations.len());

            let surveys = dat_file.surveys();
            let total_shots: usize = surveys.iter().map(|s| s.shots.len()).sum();
            println!("    Surveys: {}", surveys.len());
            println!("    Total shots: {total_shots}");

            for survey in surveys {
                println!(
                    "      - {} ({}) - {} shots",
                    survey.name,
                    survey.date,
                    survey.shots.len()
                );
            }
            println!();
        }
    } else {
        for dat_file in &project.survey_files {
            println!("  File: {}", dat_file.file_path.display());
            println!("    Datum: {:?}", dat_file.file_state.datum);
            let loc = &dat_file.file_state.base_location;
            println!(
                "    Base location: UTM Zone {}, E:{:.3} N:{:.3} Elev:{:.3}m",
                loc.zone,
                loc.east_north_elevation.easting,
                loc.east_north_elevation.northing,
                loc.east_north_elevation.up
            );
            println!("    Fixed stations: {}", dat_file.project_stations.len());
            println!("    (use --load to see survey details)");
            println!();
        }
    }

    Ok(())
}

fn inspect_survey(path: &PathBuf, verbose: bool) -> miette::Result<()> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| miette::miette!("Failed to read file {}: {}", path.display(), e))?;

    let surveys = Survey::parse_dat_file(&contents)
        .map_err(|e| miette::miette!("Failed to parse survey file: {}", e))?;

    println!("Survey file: {}", path.display());
    println!("  Surveys: {}", surveys.len());
    println!();

    for survey in &surveys {
        println!("Survey: {} - {}", survey.cave_name, survey.name);
        println!("  Date: {}", survey.date);
        println!("  Team: {}", survey.team);
        println!("  Declination: {:.2}°", survey.parameters.declination);
        println!("  Shots: {}", survey.shots.len());

        if let Some(comment) = &survey.comment {
            println!("  Comment: {comment}");
        }

        if verbose && !survey.shots.is_empty() {
            println!("  Shot data:");
            println!(
                "    {:>8} {:>8} {:>8} {:>8} {:>8}",
                "From", "To", "Length", "Azimuth", "Incl"
            );
            for shot in &survey.shots {
                println!(
                    "    {:>8} {:>8} {:>8.2} {:>8.2} {:>8.2}",
                    shot.from, shot.to, shot.length, shot.azimuth, shot.inclination
                );
            }
        }
        println!();
    }

    Ok(())
}

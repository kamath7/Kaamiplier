mod actions;
mod config;

use config::ProjectConfig;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Kaamiplier log: Usage <build|run|clean|package> <project_path>");
        return;
    }

    let action = &args[1];
    let project_path = &args[2];

    let config_path = format!("{}/kaamiplier.toml", project_path);

    let config_contents = match fs::read_to_string(&config_path) {
        Ok(text) => text,
        Err(error) => {
            println!(
                "Kaamiplier ERROR: Cannot read recipe card at '{}'.",
                config_path
            );
            println!("System reason: {}", error);
            println!("Are you sure this is a valid Kaamiplier project?");
            std::process::exit(1);
        }
    };

    let config: ProjectConfig = match toml::from_str(&config_contents) {
        Ok(parsed_config) => parsed_config,
        Err(error) => {
            println!("Kaamiplier ERROR: Your kaamiplier.toml file is corrupted!");
            println!("Parser reason: {}", error);
            std::process::exit(1);
        }
    };

    println!("Kaamiplier log: Processing project '{}'...", config.name);

    if action == "build" {
        actions::build_project(&config, project_path);
    } else if action == "run" {
        actions::run_project(&config, project_path);
    } else if action == "clean" {
        actions::clean_project(&config, project_path);
    } else if action == "package" {
        actions::package_project(&config, project_path);
    } else {
        println!("Kaamiplier log: Unknown action '{}'", action);
    }
}

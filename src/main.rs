use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct ProjectConfig {
    name: String,
    #[allow(dead_code)]
    language: String,
    source: String,
    output: String,
    main_class: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Kaamiplier log: Usage <build|run|clean> <project_path>");
        return;
    }

    let action = &args[1];
    let project_path = &args[2];

    let config_path = format!("{}/kaamiplier.toml", project_path);

    let config_contents = match fs::read_to_string(&config_path) {
        Ok(text) => text,
        Err(error) => {
            println!("Kaamiplier ERROR: Cannot read recipe card at '{}'.", config_path);
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
        build_project(&config, project_path);
    } else if action == "run" {
        run_project(&config, project_path);
    } else if action == "clean" {
        clean_project(&config, project_path);
    } else {
        println!("Kaamiplier log: Unknown action '{}'", action);
    }
}
fn build_project(config: &ProjectConfig, project_path: &str) {
    println!("Kaamiplier log: Building {}...", project_path);

    let output_dir = format!("{}/{}", project_path, config.output);
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    let source_dir = format!("{}/{}", project_path, config.source);
    let mut java_files: Vec<String> = Vec::new();
    let entries = fs::read_dir(&source_dir).expect("Failed to read source directory");

    for entry in entries {
        let entry = entry.expect("Failed to read file entry");
        let path = entry.path();
        let path_string = path.to_string_lossy().to_string();

        if path_string.ends_with(".java") {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let class_file_name = file_name.replace(".java", ".class");
            let class_file_path = format!("{}/{}", output_dir, class_file_name);

            let mut should_compile = true;

            if let Ok(class_meta) = fs::metadata(&class_file_path) {
                let java_meta = fs::metadata(&path).unwrap();
                let class_time = class_meta.modified().unwrap();
                let java_time = java_meta.modified().unwrap();

                if java_time <= class_time {
                    should_compile = false;
                }
            }

            if should_compile {
                java_files.push(path_string);
            } else {
                println!("Kaamiplier log: Skipping {} (up to date)", file_name);
            }
        }
    }

    if java_files.is_empty() {
        println!("Kaamiplier log: All files are up to date. Nothing to do!");
    } else {
        let status = Command::new("javac")
            .arg("-d")
            .arg(&output_dir)
            .args(&java_files)
            .status()
            .expect("Failed to spawn javac");

        if status.success() {
            println!("Kaamiplier log: Build success");
        } else {
            println!("Kaamiplier log: Build failed");
        }
    }
}

fn run_project(config: &ProjectConfig, project_path: &str) {
    println!("Kaamiplier log: Running {}...", project_path);

    let class_path = format!("{}/{}", project_path, config.output);
    let status = Command::new("java")
        .arg("-cp")
        .arg(class_path)
        .arg(&config.main_class)
        .status()
        .expect("Failed to spawn java");

    if status.success() {
        println!("Kaamiplier log: Run finished successfully!");
    } else {
        println!("Kaamiplier log: Run failed!");
    }
}

fn clean_project(config: &ProjectConfig, project_path: &str) {
    println!("Kaamiplier log: Cleaning {} ...", project_path);
    let output_dir = format!("{}/{}", project_path, config.output);
    
    if Path::new(&output_dir).exists() {
        match fs::remove_dir_all(&output_dir) {
            Ok(_) => println!("Kaamiplier log: Cleaned successfully! Deleted {}", output_dir),
            Err(error) => println!("Kaamiplier log: Failed to clean: {}", error),
        }
    } else {
        println!("Kaamiplier log: Nothing to clean. The build directory doesn't exist.");
    }
}
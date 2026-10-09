use serde::Deserialize;
use std::env;
use std::fs;
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
        println!("Kaamiplier log: Usage <build|run> <project_path>");
        return;
    }

    let action = &args[1];
    let project_path = &args[2];

    let config_path = format!("{}/kaamiplier.toml", project_path);
    let config_contents = fs::read_to_string(&config_path)
        .expect("Kaamiplier log:Failed to read kaamiplier.toml. Does it exist?");

    let config: ProjectConfig = toml::from_str(&config_contents)
        .expect("Kaamiplier log:Failed to parse TOML configuration");

    // We'll use config.name here just to make the compiler happy about 'name' being used!
    println!("Kaamiplier log: Processing project '{}'...", config.name);

    if action == "build" {
        println!("kaamiplier log: Building {} ", project_path);

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
                java_files.push(path_string);
            }
        }

        if java_files.is_empty() {
            println!("Kaamiplier log: No .java files found in {}!", source_dir);
            return;
        }
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
    } else if action == "run" {
        println!("kaamiplier log: Running {}..", project_path);

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
    } else {
        println!("Kaamiplier: Unknown action '{}'", action);
    }
}

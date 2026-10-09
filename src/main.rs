use serde::Deserialize;
use std::env;
use std::fs;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct ProjectConfig {
    name: String,
    language: String,
    source: String,
    output: String,
    main_class: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Kaamiplier log: <build|run> <project_path>");
        return;
    }

    let action = &args[1];
    let project_path: &String = &args[2];

    let config_path = format!("{}/kaamiplier.toml", project_path);

    let config_contents = fs::read_to_string(&config_path)
        .expect("Kaamiplier log: Failed to read kaamiplier.toml. Does it exist?");

    let config: ProjectConfig = toml::from_str(&config_contents)
        .expect("Kaamiplier log: Failed to parse TOML configuration");

    if action == "build" {
        println!("kaamiplier log: Building {} ", project_path);

        let java_file = format!("{}/{}/Main.java", project_path, config.source);
        let status = Command::new("javac")
            .arg(java_file)
            .status()
            .expect("Kaamiplier log: Failed to spawn javac");

        if status.success() {
            println!("Kaamiplier log: Build success");
        } else {
            println!("Kaamiplier log: Build failed");
        }
    } else if action == "run" {
        println!("kaamiplier log: Running {}..", project_path);

        let class_path = format!("{}/{}", project_path, config.source);

        let status = Command::new("java")
            .arg("-cp")
            .arg(class_path)
            // Use config.main_class instead of hardcoded "Main"
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

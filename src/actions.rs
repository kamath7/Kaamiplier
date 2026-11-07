use crate::config::ProjectConfig; 
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn build_project(config: &ProjectConfig, project_path: &str) {
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

pub fn run_project(config: &ProjectConfig, project_path: &str) {
    build_project(config, project_path);

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

pub fn clean_project(config: &ProjectConfig, project_path: &str) {
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

pub fn package_project(config: &ProjectConfig, project_path: &str) {
    build_project(config, project_path);
    
    println!("Kaamiplier log: Packaging {} into a To-Go Box (JAR)...", project_path);
    
    let output_dir = format!("{}/{}", project_path, config.output);
    let jar_name = format!("{}.jar", config.name);
    let jar_path = format!("{}/{}", output_dir, jar_name);

    let status_result = Command::new("jar")
        .arg("cfe")
        .arg(&jar_path)
        .arg(&config.main_class)
        .arg("-C")
        .arg(&output_dir)
        .arg(".") 
        .status();

    match status_result {
        Ok(status) => {
            if status.success() {
                println!("Kaamiplier log: Successfully packaged into {}", jar_path);
            } else {
                println!("Kaamiplier log: Packaging failed! The jar tool reported an error.");
            }
        }
        Err(error) => {
            println!("Kaamiplier log: ERROR - Could not find the 'jar' tool on your system!");
            println!("System reason: {}", error);
            println!("Hint: Make sure your Java JDK 'bin' folder is added to your Windows PATH.");
        }
    }
}
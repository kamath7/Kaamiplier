use std::process::Command;

use std::env;

fn main() {
    println!("Compile the java file");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Kaamiplier log: <build|run> <project_path>");
        return;
    }

    let action = &args[1];
    let project_path: &String = &args[2];

    if action == "build" {
        println!("kaamiplier log: Building {} ", project_path);

        let java_file = format!("{}/src/Main.java", project_path);
        let status = Command::new("javac")
            .arg(java_file)
            .status()
            .expect("Failed to spawn javac");

        if status.success() {
            println!("Kaamiplier log: Build success");
        } else {
            println!("Kaamiplier log: Build failed");
        }
    } else if action == "run" {
        println!("kaamiplier log: Running {}..", project_path);

        let class_path = format!("{}/src", project_path);
        let status = Command::new("java")
            .arg("-cp")
            .arg(class_path)
            .arg("Main") // The name of our Java class
            .status()
            .expect("Failed to spawn java");

        if status.success() {
            println!("Kaamiplier: Run finished successfully!");
        } else {
            println!("Kaamiplier: Run failed!");
        }
    } else {
        println!("Kaamiplier: Unknown action '{}'", action);
    }
}

use std::process::Command;

fn main() {
    println!("Compile the java file");


    let status = Command::new("javac")
        .arg("examples/hello/src/Main.java")
        .status()
        .expect("Failed to spawn javac");

    if status.success() {
        println!("kaamiplier log: Compilation success!");
    } else {
        println!("Kaamiplier log: Compliation failed!");
    }

}

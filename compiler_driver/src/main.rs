use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
fn main() {
    //compiler driver func as cli
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file.c>", args[0]);
        std::process::exit(1);
    }
    let source_file = Path::new(&args[1]);

    if !source_file.exists() {
        eprintln!("Error: Source file '{}' does not exist", source_file.display());
        std::process::exit(1);
    }

    compiler_driver(source_file);
}
fn compiler_driver(source_file: &Path){
    let preprocessed_file = preprocess_source_file(source_file);
    let assembly_file = compile_preprocessed_file(&preprocessed_file);
    assemble_to_object_file(&assembly_file);
}
fn preprocess_source_file(file_path: &Path)-> PathBuf {
    let input_file = file_path.to_str().unwrap();
    //preprocessed file name is the same as input file name with .i extension
    let preprocessed_file = file_path.with_extension("i");
    let preprocessed_file_str = preprocessed_file.to_str().unwrap();

    let status = Command::new("gcc")
        .args(&["-E", "-P", input_file, "-o", preprocessed_file_str])
        .status()
        .expect("failed to execute gcc");

    if status.success() {
        println!("Preprocessing succeeded!");
    } else {
        println!("Preprocessing failed.");
    }
    preprocessed_file
}
fn compile_preprocessed_file(preprocessed_file: &Path) -> PathBuf {
    let input_file = preprocessed_file.to_str().unwrap();
    let output_file = preprocessed_file.with_extension("s");
    if output_file.exists() {
        std::fs::remove_file(&output_file).expect("Failed to delete existing assembly file");
    }
    let output_file_str = output_file.to_str().unwrap();

    let status = Command::new("gcc")
        .args(&["-S", input_file, "-o", output_file_str])
        .status()
        .expect("failed to execute gcc");
   
    if status.success() {
         //delete preprocessed file
        if preprocessed_file.exists() {
            std::fs::remove_file(preprocessed_file).expect("Failed to delete preprocessed file");
        }
        println!("Compilation to assembly succeeded!");
    } else {
        println!("Compilation to assembly failed.");
    }
    output_file 
}
fn assemble_to_object_file(assembly_file: &Path){
    let input_file = assembly_file.to_str().unwrap();
    let output_file = assembly_file.with_extension("o");
    if output_file.exists() {
        std::fs::remove_file(&output_file).expect("Failed to delete existing object file");
    }
    let output_file_str = output_file.to_str().unwrap();


    let status = Command::new("gcc")
        .args(&["-c", input_file, "-o", output_file_str])
        .status()
        .expect("failed to execute gcc");

    if status.success() {
        //delete assembly file
        if assembly_file.exists() {
            std::fs::remove_file(assembly_file).expect("Failed to delete assembly file");
        }
        println!("Assembly to object file succeeded!");
    } else {
        println!("Assembly to object file failed.");
    }
}
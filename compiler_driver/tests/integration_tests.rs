use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_simple_c_file_compilation() {
    let test_file = "tests/test_files/simple_return.c";
    let output_file = "tests/test_files/simple_return.o";

    // Clean up any existing output file
    let _ = fs::remove_file(output_file);

    // Run the compiler driver
    let output = Command::new("cargo")
        .args(&["run", "--", test_file])
        .output()
        .expect("Failed to execute compiler driver");

    // Check that the command succeeded
    assert!(output.status.success(), "Compiler driver failed: {}", String::from_utf8_lossy(&output.stderr));

    // Check that the object file was created
    assert!(Path::new(output_file).exists(), "Object file was not created");

    // Clean up
    let _ = fs::remove_file(output_file);
}

#[test]
fn test_c_file_with_includes() {
    let test_file = "tests/test_files/with_includes.c";
    let output_file = "tests/test_files/with_includes.o";

    // Clean up any existing output file
    let _ = fs::remove_file(output_file);

    // Run the compiler driver
    let output = Command::new("cargo")
        .args(&["run", "--", test_file])
        .output()
        .expect("Failed to execute compiler driver");

    // Check that the command succeeded
    assert!(output.status.success(), "Compiler driver failed: {}", String::from_utf8_lossy(&output.stderr));

    // Check that the object file was created
    assert!(Path::new(output_file).exists(), "Object file was not created");

    // Clean up
    let _ = fs::remove_file(output_file);
}

#[test]
fn test_invalid_c_file_compilation() {
    let test_file = "tests/test_files/invalid_syntax.c";
    let output_file = "tests/test_files/invalid_syntax.o";

    // Clean up any existing output file
    let _ = fs::remove_file(output_file);

    // Run the compiler driver
    let output = Command::new("cargo")
        .args(&["run", "--", test_file])
        .output()
        .expect("Failed to execute compiler driver");

    // The compilation should fail due to syntax errors
    // We expect the process to exit with success (the driver ran)
    // but GCC should have failed internally
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that compilation failure was detected
    assert!(stdout.contains("failed") || !output.status.success(),
        "Expected compilation to fail for invalid syntax");

    // Object file should not be created
    assert!(!Path::new(output_file).exists(), "Object file should not be created for failed compilation");
}

#[test]
fn test_compiler_driver_with_no_args() {
    // Run the compiler driver without arguments
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to execute compiler driver");

    // Should fail and show usage message
    assert!(!output.status.success(), "Should fail when no arguments provided");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"), "Should show usage message");
}

#[test]
fn test_compiler_driver_with_nonexistent_file() {
    let test_file = "tests/test_files/nonexistent.c";

    // Run the compiler driver with non-existent file
    let output = Command::new("cargo")
        .args(&["run", "--", test_file])
        .output()
        .expect("Failed to execute compiler driver");

    // Should fail
    assert!(!output.status.success(), "Should fail when file doesn't exist");
}

#[test]
fn test_intermediate_files_are_cleaned_up() {
    let test_file = "tests/test_files/simple_return.c";
    let preprocessed_file = "tests/test_files/simple_return.i";
    let assembly_file = "tests/test_files/simple_return.s";
    let output_file = "tests/test_files/simple_return.o";

    // Clean up any existing files
    let _ = fs::remove_file(preprocessed_file);
    let _ = fs::remove_file(assembly_file);
    let _ = fs::remove_file(output_file);

    // Run the compiler driver
    let output = Command::new("cargo")
        .args(&["run", "--", test_file])
        .output()
        .expect("Failed to execute compiler driver");

    // Check that the command succeeded
    assert!(output.status.success(), "Compiler driver failed: {}", String::from_utf8_lossy(&output.stderr));

    // Check that intermediate files were cleaned up
    assert!(!Path::new(preprocessed_file).exists(), "Preprocessed file should be cleaned up");
    assert!(!Path::new(assembly_file).exists(), "Assembly file should be cleaned up");

    // Object file should exist
    assert!(Path::new(output_file).exists(), "Object file should be created");

    // Clean up
    let _ = fs::remove_file(output_file);
}
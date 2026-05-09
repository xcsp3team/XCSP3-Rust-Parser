use std::process::Command;

fn run_file(path: &str) {
    let binary = env!("CARGO_BIN_EXE_xcsp3-rust"); // replace with your binary name    let output = Command::new("cargo")
    let output = Command::new(binary)
        .args(["run", "--", path])
        //.stdout(std::process::Stdio::null())
        //.stderr(std::process::Stdio::null())
        .output()
        .expect("failed to run");

    assert!(output.status.success(), "failed on file: {}", path);
}

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

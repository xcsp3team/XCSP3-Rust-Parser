#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    fn run_on_dir(dir: &str) {
        let entries = fs::read_dir(dir).expect(&format!("cannot read dir {}", dir));
        for entry in entries {
            println!("{:?}", entry);
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) == Some("xml") {
                // adjust extension
                let output = Command::new("cargo")
                    .args(["run", "--", path.to_str().unwrap()])
                    .output()
                    .expect("failed to run");
                assert!(
                    output.status.success(),
                    "failed on file {:?}\nstdout: {}\nstderr: {}",
                    path,
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }

    #[test]
    fn test_dir_cop22_25() {
        run_on_dir("instances/cop22_25");
    }

    /*#[test]
    fn test_dir_objectives() {
        run_on_dir("examples/objectives");
    }

    #[test]
    fn test_dir_other() {
        run_on_dir("examples/other");
    }
     */
}

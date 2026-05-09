use std::fs;
use std::io::Write;

fn generate_tests_for_dir(f: &mut fs::File, dir: &str, prefix: &str) {
    let entries = fs::read_dir(dir).expect(&format!("cannot read dir {}", dir));
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("xml") {
            let name = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase()
                .replace("-", "_")
                .replace(" ", "_");
            writeln!(f, "#[test]").unwrap();
            writeln!(f, "fn {}_{}() {{", prefix, name).unwrap();
            writeln!(f, "    run_file({:?});", path.to_str().unwrap()).unwrap();
            writeln!(f, "}}").unwrap();
        }
    }
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = format!("{}/generated_tests.rs", out_dir);
    let mut f = fs::File::create(&dest).unwrap();

    generate_tests_for_dir(&mut f, "instances/cop22_25", "cop_22_25");
    //generate_tests_for_dir(&mut f, "examples/objectives", "objectives");
    //generate_tests_for_dir(&mut f, "examples/other", "other");
}

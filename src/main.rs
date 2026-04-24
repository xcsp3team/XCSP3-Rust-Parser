use std::env;

use xcsp3_rust::example_callback::PrintingSolver;
use xcsp3_rust::xcsp_runner::XcspRunner;

fn main() {
    let xml_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "instances/my-example.xml".to_string());

    let mut solver = PrintingSolver::new();

    match XcspRunner::run(&xml_file, &mut solver) {
        Ok(_) => println!("Successful parsing!"),
        Err(e) => eprintln!("Error during parsing: {}", e),
    }
}

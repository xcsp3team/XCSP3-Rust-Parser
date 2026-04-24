use xcsp3_rust::example_callback::PrintingSolver;
use xcsp3_rust::xcsp_runner::XcspRunner;


fn main() {
    let xml_file = "../instances/my-example.xml";

    let mut solver = PrintingSolver::new();

    match XcspRunner::run(xml_file, &mut solver) {
        Ok(_) => println!("Successful parsing!"),
        Err(e) => eprintln!("Error during parsing: {}", e),
    }
}

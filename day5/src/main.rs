use std::process;

fn main() {
    if let Err(e) = day5::run() {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

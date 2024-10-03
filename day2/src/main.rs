use std::process;

/// Find the elf carrying the most calories
fn main() {
    if let Err(e) = day2::run() {
        eprintln!("Application error: {e}");
        process::exit(1)
    };
}

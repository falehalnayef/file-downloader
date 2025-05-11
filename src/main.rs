mod cli;
mod runner;
mod utils;
fn main() {
    println!("Hello, FD!");

    if let Err(e) = runner::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

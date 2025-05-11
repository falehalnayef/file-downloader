use crate::cli;
use crate::utils;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let (url, threads, output_file) = cli::get_args()?;
    let u = utils::validate_url(&url)?;

    utils::is_valid_file_url(&url)?;

    println!("URL: {}", url);
    println!("Threads: {}", threads);
    println!("Output file: {}", output_file);

    Ok(())
}

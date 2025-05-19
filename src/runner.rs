use crate::cli;
use crate::utils;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let (url, threads, output_file) = cli::get_args()?;
    let u = utils::validate_url(&url)?;

    let file_info = utils::get_file_info(&url)?;
    if !file_info.supports_ranges {
        println!("Server does not support partial download");
    }

    println!("URL: {}", url);
    println!("Threads: {}", threads);
    println!("Output file: {}", output_file);

    Ok(())
}

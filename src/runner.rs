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

    let ranges = utils::calculate_ranges(file_info.content_length, threads);
    let handles: Vec<_> = ranges
        .iter()
        .enumerate()
        .map(|(i, &range)| {
            let url = url.clone();
            std::thread::spawn(move || {
                utils::download_chunk(&url, range, i).expect("Failed to download chunk");
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("URL: {}", url);
    println!("Threads: {}", threads);
    println!("Output file: {}", output_file);

    Ok(())
}

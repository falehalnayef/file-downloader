use std::env;
use std::error::Error;

pub fn get_args() -> Result<(String, usize, String), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: <program> <url> [threads] [output_file]".into());
    }

    let url = args[1].clone();

    let threads = if args.len() > 2 {
        args[2].parse().map_err(|_| "Invalid number for threads")?
    } else {
        1
    };

    let output_file = if args.len() > 3 {
        args[3].clone()
    } else {
        String::from("output.file")
    };

    Ok((url, threads, output_file))
}

use std::{env, path::Path, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Failed to parse arguments.\nUsage: {} <path>",
            args.get(0).unwrap()
        );
        process::exit(1);
    }

    let path = Path::new(args.get(1).unwrap());
    if !path.exists() {
        eprintln!("The path you entered doesn't exist.");
        process::exit(1);
    }

    println!("The path {:?} is being watched.", path.file_name());

    Ok(())
}

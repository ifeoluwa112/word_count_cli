mod args;
mod counter;
mod error;

use args::Args;
use counter::count_file;
use error::AppError;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let args = Args::parse("").map_err(AppError::Args)?;

    let result = count_file(&args.file_path)?;

    println!("Lines: {}", result.lines);
    println!("Words: {}", result.words);
    println!("Characters: {}", result.characters);

    Ok(())
}

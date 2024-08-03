use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to Dropbox files to move
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("path: {:?}", args.path)
}

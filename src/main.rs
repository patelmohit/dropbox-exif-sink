use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The path to move data from
    from_path: std::path::PathBuf,
    // The path to move data to
    to_path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    println!("from_path: {:?}, to_path: {:?}", args.from_path, args.to_path);
    let from_is_dir = args.from_path.is_dir();
    let to_is_dir = args.to_path.is_dir();
    if !from_is_dir || !to_is_dir {
        
        if !from_is_dir {
            println!("{:?} is not a valid directory", args.from_path);
        }
        if !to_is_dir {
            println!("{:?} is not a valid directory", args.to_path);
        }
        panic!("Failed to run the program - both paths are not directories.")

    } else {
        println!("yay we have 2 dirs!")
    }
    // let copy_result = std::fs::copy(args.from_path, args.to_path);
    // match copy_result {
    //     Ok(content) => { println!("copy success? {}", content);} 
    //     Err(error) => { println!("ahhhhhhhhhhh couldn't copy {}", error)}
    // }
}

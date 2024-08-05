use std::{fs::{create_dir_all, rename}, io};

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The path to move data from
    from_path: std::path::PathBuf,
    // The path to move data to
    to_path: std::path::PathBuf
}

// 
fn merge_dir(from_dir: std::path::PathBuf, to_dir: std::path::PathBuf) -> io::Result<()> {
    let read_dir = from_dir.read_dir()?;

    for read_entry_result in read_dir {
        let read_entry = read_entry_result?;
        let file_type =  read_entry.file_type()?;
        let read_name = read_entry.file_name();
        if file_type.is_file() {
            // Move file over to the write directory
            let read_path = read_entry.path();
            let write_folder = to_dir.as_path();
            let write_path = write_folder.join(&read_name);
            if write_path.exists() {
                println!("{:?} already exists, not moving", write_path);
                continue;
            }
            println!("moving file {:?} from {:?} to {:?}", read_name, read_path, write_path);
            create_dir_all(write_folder)?;
            rename(read_path, write_path)?;
        } else if file_type.is_dir() {
            let read_path = read_entry.path();
            let write_path = to_dir.as_path().join(read_name);
            merge_dir(read_path, write_path)?;
        } else if file_type.is_symlink() {
            println!("support for symlink not available for file {:?}", read_name);          
        } else {
            println!("unknown file type {:?} for file {:?}", file_type, read_name);
        }
        continue;
    }
    Ok(())
}

fn main() {
    let args = Cli::parse();
    let from_is_dir = args.from_path.is_dir();
    let to_is_dir = args.to_path.is_dir();
    if !from_is_dir || !to_is_dir {
        if !from_is_dir {
            println!("{:?} is not a valid directory", args.from_path);
        }
        if !to_is_dir {
            println!("{:?} is not a valid directory", args.to_path);
        }
        panic!("failed to run the program - both paths are not directories.");
    }

    match merge_dir(args.from_path, args.to_path) {
        Ok(()) => {println!("success merging dirs!")}
        Err(e) => {panic!("failed to merge dirs - {}", e)}
    }
}

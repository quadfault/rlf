// main.rs - Recursively list files (and just files!) in a directory hierarchy.
// Written by quadfault
// 10/25/18

use std::io;
use std::path::Path;
use std::process;

fn main() {
    // dir = current working directory
    // for each file in dir:
    //     if file is a directory:
    //         recur with dir = file
    //     else (normal or special file):
    //         print path of file relative to current working directory

    if let Err(e) = recursively_list_files_in(Path::new(".")) {
        eprintln!("error: {}", e);

        process::exit(1);
    }
}

fn recursively_list_files_in(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                recursively_list_files_in(&path)?;
            } else if let Some(pathname) = path.to_str() {
                println!("{}", pathname);
            }
        }
    }

    Ok(())
}

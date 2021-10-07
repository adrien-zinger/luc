use std::time::SystemTime;
use std::{env, fs};
use std::vec::Vec;

fn read_dir(path: String) -> Vec::<(String, SystemTime, usize)> {
    let rd = fs::read_dir(path.to_owned());
    let mut file = Vec::new();
    if rd.is_err() {
        eprintln!("Luc! error reading file {}", path);
    } else {
        for entry in rd.unwrap().flatten() {
            let path = entry.path().display().to_string();
            let metadata = fs::metadata(&path).unwrap();
            let last_modified = metadata.modified().unwrap();
            if metadata.is_file() {
                file.push((path, last_modified, metadata.len() as usize));
                println!(
                    "Last modified: {:?} seconds, size: {:?} bytes, filename: {:?}",
                    last_modified,
                    metadata.permissions().readonly(),
                    metadata.len()
                );
            } else {
                file.append(&mut read_dir(path));
            }
        }
    }
    file
}

fn _recurs_stat() -> Vec::<(String, SystemTime, usize)> {
    let current_dir = env::current_dir().unwrap();
    read_dir(current_dir.display().to_string())
}

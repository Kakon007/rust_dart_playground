use std::fs;
use std::path::Path;

fn main() {
    let folder_path = Path::new("/home/jahid007/rust_play_folder/Jahid");
    let file_path = Path::new("/home/jahid007/rust_play_folder/Jahid/file.txt");


    match fs::create_dir(folder_path) {
        Ok(_) => println!("Folder created"),
        Err(err) => println!("Failed to create folder: {}", err),
    }

    match fs::write(file_path, "Hello, world!") {
        Ok(_) => println!("File created"),
        Err(err) => println!("Failed to create file: {}", err),
    }
}
// // use std::{time::Duration, fs};
// use std::{path::Path, fs};
// // use notify::{RecommendedWatcher, RecursiveMode, Result, recommended_watcher, Watcher};
// // use std::thread;

// fn main() {
//     let folder_path = Path::new("/home/jahid007/rust_play_folder/Jahid");
//     let file_path = Path::new("/home/jahid007/rust_play_folder/Jahid/file.txt");

//     match fs::create_dir(folder_path) {
//         Ok(_) => println!("Folder created"),
//         Err(err) => println!("Failed to create folder: {}", err),
//     }

//     match fs::write(file_path, "Hello, world!") {
//         Ok(_) => println!("File created"),
//         Err(err) => println!("Failed to create file: {}", err),
//     }
// }

// fn main() -> Result<()> {
//     // Automatically select the best implementation for your platform.
//     let mut watcher: RecommendedWatcher = recommended_watcher(|res| {
//         match res {
//            Ok(event) => println!("event: {:?}", event),
//            Err(e) => println!("watch error: {:?}", e),
//         }
//     })?;

//     watcher.watch(Path::new("/home/jahid007/rust_play_folder/Jahid"), RecursiveMode::Recursive)?;

//     thread::sleep(Duration::from_secs(10));
//     Ok(())
// }

// use std::io;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();
//     io::stdin()
//     .read_line(&mut guess)
//     .expect("Failed to read line");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("The secret number is: {secret_number}");

//     println!("You guessed: {guess}");
// }

use fs_extra::dir::CopyOptions;

// fn main(){
//   let from= "/home/jahid007/rust_play_folder/Jahid";
//     let to = "/home/jahid007/rust_play_folder/Jahid2";
//     let options = CopyOptions::new();
//     fs_extra::dir::copy(from, to, &options).unwrap();
// }

use std::path::Path;

fn main() {
    let path = "/home/jahid007/rust_play_folder/Jahid";
    let from = "/home/jahid007/rust_play_folder/Jahid";
    let to = "/home/jahid007/rust_play_folder/jahid2";
    let options = CopyOptions::new();
    if Path::new(path).exists() && Path::new(path).is_dir() {
        println!("The folder exists.");
        fs_extra::dir::copy(from, to, &options).unwrap();
    } else {
        println!("The folder does not exist.");
    }
}

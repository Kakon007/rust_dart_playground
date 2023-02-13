extern crate notify;

use std::time::Duration;
use std::path::Path;
use notify::{RecommendedWatcher, RecursiveMode, Result, recommended_watcher, Watcher};
use std::thread;




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

fn main() -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher: RecommendedWatcher = recommended_watcher(|res| {
        match res {
           Ok(event) => println!("event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("/home/jahid007/rust_play_folder/Jahid"), RecursiveMode::Recursive)?;

    thread::sleep(Duration::from_secs(10));
    Ok(())
}
// // // use std::{time::Duration, fs};
// // use std::{path::Path, fs};
// // // use notify::{RecommendedWatcher, RecursiveMode, Result, recommended_watcher, Watcher};
// // // use std::thread;

// // fn main() {
// //     let folder_path = Path::new("/home/jahid007/rust_play_folder/Jahid");
// //     let file_path = Path::new("/home/jahid007/rust_play_folder/Jahid/file.txt");

// //     match fs::create_dir(folder_path) {
// //         Ok(_) => println!("Folder created"),
// //         Err(err) => println!("Failed to create folder: {}", err),
// //     }

// //     match fs::write(file_path, "Hello, world!") {
// //         Ok(_) => println!("File created"),
// //         Err(err) => println!("Failed to create file: {}", err),
// //     }
// // }

// // fn main() -> Result<()> {
// //     // Automatically select the best implementation for your platform.
// //     let mut watcher: RecommendedWatcher = recommended_watcher(|res| {
// //         match res {
// //            Ok(event) => println!("event: {:?}", event),
// //            Err(e) => println!("watch error: {:?}", e),
// //         }
// //     })?;

// //     watcher.watch(Path::new("/home/jahid007/rust_play_folder/Jahid"), RecursiveMode::Recursive)?;

// //     thread::sleep(Duration::from_secs(10));
// //     Ok(())
// // }

// // use std::io;
// // use rand::Rng;

// // fn main() {
// //     println!("Guess the number!");

// //     println!("Please input your guess.");

// //     let mut guess = String::new();
// //     io::stdin()
// //     .read_line(&mut guess)
// //     .expect("Failed to read line");

// //     let secret_number = rand::thread_rng().gen_range(1..=100);

// //     println!("The secret number is: {secret_number}");

// //     println!("You guessed: {guess}");
// // }


// // fn main(){
// //   let from= "/home/jahid007/rust_play_folder/Jahid";
// //     let to = "/home/jahid007/rust_play_folder/Jahid2";
// //     let options = CopyOptions::new();
// //     fs_extra::dir::copy(from, to, &options).unwrap();
// // }

// use std::path::Path;
// use std::fs;
// use std::io;
// fn main() {
//     // let path = "/home/jahid007/rust_play_folder/Jahid";
//     // let from = "/home/jahid007/rust_play_folder/Jahid";
//     // let to = "/home/jahid007/rust_play_folder/jahid2";
//    // let options = CopyOptions::new();
//    let source = Path::new("/home/jahid007/rust_play_folder/Jahid");
//    let destination = Path::new("/home/jahid007/rust_play_folder/jahid2");

//    copy_dir(source, destination).unwrap();
   
// }




// fn copy_dir(source: &Path, destination: &Path) -> io::Result<()> {
//     if source.is_dir() {
//         fs::create_dir_all(destination)?;
//         for entry in fs::read_dir(source)? {
//             let entry = entry?;
//             let entry_path = entry.path();
//             let dest_path = destination.join(entry.file_name());
//             if entry_path.is_dir() {
//                 copy_dir(&entry_path, &dest_path)?;
//             } else {
//                 fs::copy(&entry_path, &dest_path)?;
//             }
//         }
//     } else {
//         fs::copy(source, destination)?;
//     }
//     Ok(())
// }


use std::sync::mpsc::channel;
use std::thread;
use std::{path::Path, time::SystemTime};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

fn main() {
    // let path = std::env::args()
    //     .nth(1)
    //     .expect("Argument 1 needs to be a path");
    let path_string = "/home/jahid007/rust_play_folder/Jahid";
    let path = Path::new(path_string);
    println!("Path: {:?}", path);

    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (sp, rp) = channel();

    let mut watcher = RecommendedWatcher::new(sp, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    let handle = thread::spawn(move || {
        while let Ok(event) = rp.recv() {
            handle_event(&event.unwrap()).unwrap();
        }
    });

    handle.join().unwrap();

    Ok(())
}

fn handle_event(event: &notify::Event) -> notify::Result<()> {
    // let _id = SystemTime::now()
    //     .duration_since(SystemTime::UNIX_EPOCH)
    //     .unwrap()
    //     .as_millis();
    Path::new(&event.paths[0].to_str().unwrap()).file_name().unwrap().to_str().unwrap();
    if event.kind.is_create() {
        println!("File createds {}: File createds", Path::new(&event.paths[0].to_str().unwrap()).to_path_buf().to_str().unwrap());
    } else if event.kind.is_modify() {
        println!("File modifieds {}: File modifieds", Path::new(&event.paths[0].to_str().unwrap()).to_path_buf().to_str().unwrap());
    } else if event.kind.is_remove() {
        println!("File removeds {}: File removeds", Path::new(&event.paths[0].to_str().unwrap()).to_path_buf().to_str().unwrap());
    } else if event.kind.is_other() {
        println!("File accesseds {}: File accesseds", Path::new(&event.paths[0].to_str().unwrap()).to_path_buf().to_str().unwrap());
    } else {
        println!("Other events {}: Other events", Path::new(&event.paths[0].to_str().unwrap()).to_path_buf().to_str().unwrap());
    }
    println!("JBL2 {:?}", event);

    Ok(())
}




// use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher, Config};
// use std::path::Path;
// use std::sync::mpsc::channel;


// fn watch_folder<P: AsRef<Path>>(path: P) -> notify::Result<()> {
//     let (tx, rx) = channel();
//     let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;
//     watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

//     loop {
//         match rx.recv() {
//             Ok(event) => println!("{:?}", event),
//             Err(e) => println!("watch error: {:?}", e),
//         }
//     }
// }

// fn main() -> Result<()> {
//     watch_folder("/home/jahid007/rust_play_folder/Jahid")?;
//     Ok(())
// }


// use std::{path::Path, time::Duration};

// use notify::{RecursiveMode};
// use notify_debouncer_mini::new_debouncer;

// /// Example for debouncer
// fn main() {
//     // emit some events by changing a file
//     std::thread::spawn(|| {
//         let path = Path::new("test.txt");
//         let _ = std::fs::remove_file(&path);
//         loop {
//             std::fs::write(&path, b"Lorem ipsum").unwrap();
//             std::thread::sleep(Duration::from_millis(250));
//         }
//     });

//     // setup debouncer
//     let (tx, rx) = std::sync::mpsc::channel();

//     // No specific tickrate, max debounce time 2 seconds
//     let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx).unwrap();

//     debouncer
//         .watcher()
//         .watch(Path::new("/home/jahid007/rust_play_folder/Jahid"), RecursiveMode::Recursive)
//         .unwrap();

//     // print all events, non returning
//     for events in rx {
//         for e in events {
//             println!("{:?}", e);
//         }
//     }
// }

// use std::{path::Path, time::Duration};
// use notify::*;


// // example of detecting the recommended watcher kind
// fn main() {
//     let (tx, rx) = std::sync::mpsc::channel();
//     // This example is a little bit misleading as you can just create one Config and use it for all watchers.
//     // That way the pollwatcher specific stuff is still configured, if it should be used.
//     let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
//         // custom config for PollWatcher kind
//         // you 
//         let config = Config::default()
//             .with_poll_interval(Duration::from_secs(1));
//         Box::new(PollWatcher::new(tx, config).unwrap())
//     } else {
//         // use default config for everything else
//         Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
//     };

//     // watch some stuff
//     watcher
//         .watch(Path::new("/home/jahid007/rust_play_folder/Jahid"), RecursiveMode::Recursive)
//         .unwrap();

//     // just print all events, this blocks forever
//     for e in rx {
//         println!("{:?}", e);
//     }
// }
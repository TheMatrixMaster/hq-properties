use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::{path::Path, ffi::OsStr};
use std::collections::HashMap;
use std::io::prelude::*;

const FILENAMES: [(&'static str, &'static str); 3] = [
    ("UNITES_DETAILLEES.TXT", "index"),
    ("PHOTOS.TXT", "photos"),
    ("INSCRIPTIONS.TXT", "listings")
];

#[derive(Debug)]
pub struct FileWatcherError {
    pub details: String,
}

fn make_err(err: &str) -> FileWatcherError {
    FileWatcherError { details: err.to_string() }
}

fn watch<P: AsRef<Path>>(path: P) -> Result<(), FileWatcherError> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .expect("FAILURE: Could not initialize `FsEventWatcher` struct");

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)
        .expect("FAILURE: Failed to start watcher service to watch provided path");

    for res in rx {
        match res {
            Ok(event) => {                
                if event.kind.is_create() {
                    let main_path = match event.paths
                        .get(0)
                        .ok_or(make_err("Expected file path for associated create event")) {
                            Ok(v) => v,
                            Err(e) => { println!("{:}", e.details); continue; },
                        };

                    let is_file_stem_valid = match main_path
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .ok_or(make_err("Expected file path to have a valid file stem")) {
                            Ok(v) => v.starts_with("HONGQU"),
                            Err(e) => { println!("{:}", e.details); continue; },
                        };

                    let extension = match main_path
                        .extension()
                        .ok_or(make_err("Expected file path to have a valid file extension")) {
                            Ok(v) => v.to_str(),
                            Err(e) => { println!("{:}", e.details); continue; },
                        };

                    match (is_file_stem_valid, main_path.is_file(), extension) {
                        (true, true, Some("zip")) => {
                            println!("Processing new data from zip file: {:?}", main_path);
                            
                            // TODO Now we process the zip file and inform admin if failed to process file
                            let zipfile = match std::fs::File::open(main_path) {
                                Ok(v) => v,
                                Err(e) => { println!("{e}"); continue; }
                            };

                            let mut archive = match zip::ZipArchive::new(zipfile) {
                                Ok(v) => v,
                                Err(e) => { println!("{e}"); continue; }
                            };
                            
                            let mut did_fail: bool = false;
                            let mut data: HashMap<&str, Vec<u8>> = HashMap::new();
                            
                            for (file_name, key) in FILENAMES {
                                let mut main_file = match archive.by_name(file_name) {
                                    Ok(file) => file,
                                    Err(..) => {
                                        println!("{key} file '{file_name}' not found in zip!");
                                        did_fail = true;
                                        break;
                                    }
                                };

                                let mut contents = Vec::new();
                                main_file.read_to_end(&mut contents).unwrap();
                                // println!("{:?}", String::from_utf8_lossy(&contents));
                                // println!();

                                data.insert(key, contents);
                            }
                            
                            // TODO notify Stephen that process failed here
                            if did_fail { continue }

                            // Process listings
                            // let _ = parse_listings(data.get_mut("listings").unwrap());
                            // let _ = parse_listing_images(data.get_mut("photos").unwrap());
                        },
                        _ => {
                            println!("Deleting unexpected file: {:?}", main_path);
                            match std::fs::remove_file(main_path).err() {
                                Some(e) => { println!("{:}", e); continue; },
                                None => continue,
                            };
                        }
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}


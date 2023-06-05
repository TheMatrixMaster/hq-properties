use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, ErrorKind};
use std::{path::Path, ffi::OsStr};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn make_err(err: &str) -> notify::Error {
    notify::Error::new(ErrorKind::Generic(err.to_string()))
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {                
                if event.kind.is_create() {
                    let main_path = match event.paths
                        .get(0)
                        .ok_or(make_err("Expected file path for associated create event")) {
                            Ok(v) => v,
                            Err(e) => { println!("{:}", e); continue; },
                        };

                    let is_file_stem_valid = match main_path
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .ok_or(make_err("Expected file path to have a valid file stem")) {
                            Ok(v) => v.starts_with("HONGQU"),
                            Err(e) => { println!("{:}", e); continue; },
                        };

                    let extension = match main_path
                        .extension()
                        .ok_or(make_err("Expected file path to have a valid file extension")) {
                            Ok(v) => v.to_str(),
                            Err(e) => { println!("{:}", e); continue; },
                        };

                    match (is_file_stem_valid, main_path.is_file(), extension) {
                        (true, true, Some("zip")) => {
                            println!("Processing new data from zip file: {:?}", main_path);
                            // TODO Now we process the zip file
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
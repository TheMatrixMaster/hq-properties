use backend_api::listings::{NewListing, NewListingImage};
use backend_api::{establish_connection, FileWatcherError, schema::*};
use diesel::{QueryResult, DecoratableTarget};
use diesel::{RunQueryDsl, ExpressionMethods, upsert::excluded};
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};

use std::{path::Path, ffi::OsStr};
use std::collections::HashMap;
use std::io::prelude::*;

const FILENAMES: [(&'static str, &'static str); 3] = [
    ("UNITES_DETAILLEES.TXT", "index"),
    ("PHOTOS.TXT", "photos"),
    ("INSCRIPTIONS.TXT", "listings")
];

fn make_err(err: &str) -> FileWatcherError {
    FileWatcherError { details: err.to_string() }
}

fn upsert_listings(listings: Vec<NewListing>) -> QueryResult<usize> {
    use backend_api::schema::listings::*;
    let connection = &mut establish_connection();

    diesel::insert_into(listings::table)
        .values(&listings)
        .on_conflict(listings::id)
        .filter_target(updated_at.ne(excluded(updated_at)))
        .do_update()
        .set((
            price.eq(excluded(price)),
            market_st.eq(excluded(market_st)),
            updated_at.eq(excluded(updated_at))
        ))
        .execute(connection)
}

fn upsert_listing_images(listing_images: Vec<NewListingImage>) -> QueryResult<usize> {
    use backend_api::schema::listing_images::*;
    let connection = &mut establish_connection();

    diesel::insert_into(listing_images::table)
        .values(&listing_images)
        .on_conflict(listing_images::id)
        .filter_target(url.ne(excluded(url)))
        .do_update()
        .set((
            url.eq(excluded(url)),
            priority.eq(excluded(priority)),
            tag.eq(excluded(tag)),
        ))
        .execute(connection)
}

fn parse_listings(data: &mut Vec<u8>) -> Result<(), FileWatcherError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_slice());

    for record in rdr.byte_records() {
        let r = match record {
            Ok(v) => v,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        println!("{:}: {:?}", r.len(), r);
    }

    Ok(())
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
                            parse_listings(data.get_mut("listings").unwrap());
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

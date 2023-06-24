use backend_api::*;
use backend_api::listings::{NewListing, MarketStatus, NewListingImage};
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};

use chrono::NaiveDateTime;
use rocket::serde::{Deserialize};
use std::{path::Path, ffi::OsStr, process::Command};
use std::collections::HashMap;

const FILENAMES: [(&'static str, &'static str); 3] = [
    ("UNITES_DETAILLEES.TXT", "index"),
    ("PHOTOS.TXT", "photos"),
    ("INSCRIPTIONS.TXT", "listings")
];

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GeocodeAddress {
    pub suburb: Option<String>,
    pub city: Option<String>,
    pub town: Option<String>,
    // pub state: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GeocodeResp {
    address: GeocodeAddress,
}

fn make_err(err: &str) -> FileWatcherError {
    FileWatcherError { details: err.to_string() }
}

trait LogErrorTrait<T> {
    fn log_err(self, err_msg: &str) -> Option<T>;
}

impl<T> LogErrorTrait<T> for Option<T> {
    fn log_err(self, err_msg: &str) -> Option<T> {
        if self.is_none() { println!("{err_msg}"); }
        self
    }
}

pub fn parse_f64(bytes: Option<&[u8]>) -> Option<f64> {
    parse_str(bytes)
        .map(|f| f.parse::<f64>().ok())
        .flatten()
}

pub fn parse_i32(bytes: Option<&[u8]>) -> Option<i32> {
    parse_str(bytes)
        .map(|s| s.parse::<i32>().ok())
        .flatten()
}

pub fn parse_i16(bytes: Option<&[u8]>) -> Option<i16> {
    parse_str(bytes)
        .map(|s| s.parse::<i16>().ok())
        .flatten()
}

pub fn parse_str(bytes: Option<&[u8]>) -> Option<String> {
    bytes
        .map(|b| String::from_utf8_lossy(b))
        .map(|nb| nb.to_string())
        .filter(|s| !s.is_empty())
}

pub fn parse_market_st(string: Option<String>) -> Option<MarketStatus> {
    match string?.as_str() {
        "EV" => Some(MarketStatus::Sale),
        "VE" => Some(MarketStatus::Sold),
        _ => Some(MarketStatus::Expired),
        // _ => Some(MarketStatus::Rent)
    }
}

pub fn parse_datetime(string: Option<String>) -> Option<NaiveDateTime> {
    string
        .map(|s| NaiveDateTime::parse_from_str(&s, "%Y/%m/%d %T").ok())
        .flatten()
}

pub fn geocode_city_from_coords(lat: f64, long: f64) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://geocode.maps.co/reverse?lat={lat}&lon={long}");
    let resp = 
        reqwest::blocking::get(url)?
        .json::<GeocodeResp>()?;
    
    let city = resp.address.city;
    let town = resp.address.town;
    let suburb = resp.address.suburb;
    
    let ans = {
        if town.is_some() { format!("{:}", town.unwrap()) }
        else if city.is_some() && suburb.is_some() {
            format!("{:} ({:})", city.unwrap(), suburb.unwrap())
        } else if city.is_some() { city.unwrap() }
        else if suburb.is_some() { suburb.unwrap() }
        else { "Montreal".to_string() }
    };

    Ok(ans)
}

pub fn parse_address(
    h1: String,
    h2: Option<String>,
    street: String,
    no: Option<String>,
    pcode: String
) -> String {
    let house_no = h2
        .map_or(h1.clone(), |b| format!("{h1}-{b}"));
    let street_no = no
        .map_or(street.clone(), |b| format!("{street} #{b}"));

    format!("{house_no} {street_no}, {pcode}")
}

fn parse_listing_images(data: &mut Vec<u8>) -> Result<(), FileWatcherError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_slice());

    let res = rdr.byte_records().enumerate().filter_map(|(index, record)| {
        println!("Parsing photos file, row#{index}");

        let r = record
            .map_err(|e| { println!("{e}") })
            .ok()?;

        let id = parse_i32(r.get(7))?;
        let listing_id = parse_i32(r.get(0))?;
        let priority = parse_i16(r.get(1)).unwrap_or(-1);
        let tag = parse_str(r.get(3));
        let url = parse_str(r.get(6))?;

        Some(NewListingImage {
            id,
            listing_id,
            url,
            priority,
            tag
        })
    }).collect::<Vec<NewListingImage>>();

    let upserted_images = match upsert_listing_images(res) {
        Ok(v) => v,
        Err(e) => { println!("{e}"); panic!("Failed to insert listing images into database!") }
    };

    println!("Successfully upserted {upserted_images} images into the db!");

    Ok(())
}

fn parse_listings(data: &mut Vec<u8>) -> Result<(), FileWatcherError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_slice());

    let res = rdr.byte_records().enumerate().filter_map(|(index, record)| {
        println!("Parsing index file, row #{index}");

        let r = record
            .map_err(|e| { println!("{e}") })
            .ok()?;

        let id = parse_i32(r.get(0)).log_err("Failed on parse id")?;
        let bedrooms = parse_i16(r.get(82)).unwrap_or(-1);
        let bathrooms = parse_i16(r.get(85)).unwrap_or(-1);

        let area = parse_f64(r.get(68))
            .or_else(|| parse_f64(r.get(75)))
            .log_err("Failed on parse area")?;

        let price = parse_i32(r.get(6)).log_err("Failed on parse price")?;
        let market_st = parse_market_st(parse_str(r.get(115)))
            .log_err("Failed on parse market status")?;
        let updated_at = parse_datetime(parse_str(r.get(113)))
            .log_err("Failed on parse datetime")?;

        let h1 = parse_str(r.get(25)).log_err("Failed on parse house number")?;
        let h2 = parse_str(r.get(26));
        let street = parse_str(r.get(27)).log_err("Failed on parse street")?;
        let no = parse_str(r.get(28));
        let pcode = parse_str(r.get(29)).log_err("Failed on parse postal code")?;
        let address = parse_address(h1, h2, street, no, pcode);

        let latitude = parse_f64(r.get(144)).log_err("Failed on parse latitude")?;
        let longitude = parse_f64(r.get(145)).log_err("Failed on parse longitude")?;
        let city = geocode_city_from_coords(latitude, longitude)
            .map_err(|e| { println!("{e}") })
            .ok()?;

        let listing_url = parse_str(r.get(132)).log_err("Failed to parse passerelle url")?;

        Some(NewListing {
            id,
            city,
            address,
            listing_url,
            bedrooms,
            bathrooms,
            area,
            price,
            market_st,
            updated_at
        })
    }).collect::<Vec<NewListing>>();

    let upserted_listings = match upsert_listings(res) {
        Ok(v) => v,
        Err(e) => { println!("{e}"); panic!("Failed to insert listings into database!") }
    };

    println!("Successfully upserted {upserted_listings} listings into the db!");

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
                            
                            let mut did_fail: bool = false;
                            let mut data: HashMap<&str, Vec<u8>> = HashMap::new();
                            
                            for (file_name, key) in FILENAMES {

                                let main_file = match Command::new("unzip")
                                    .arg("-p")
                                    .arg(main_path)
                                    .arg(file_name)
                                    .output() {
                                        Ok(v) => v,
                                        Err(e) => {
                                            println!("Error: {e}!");
                                            did_fail = true;
                                            break;
                                        }
                                    };
                                
                                if main_file.status.success() {
                                    data.insert(key, main_file.stdout);
                                } else {
                                    println!("Error: {:?}", parse_str(Some(&main_file.stderr)));
                                    did_fail = true;
                                    break;
                                }
                            }
                            
                            // TODO notify Stephen that process failed here
                            if did_fail { continue }

                            // Process listings
                            let _ = parse_listings(data.get_mut("listings").unwrap());
                            let _ = parse_listing_images(data.get_mut("photos").unwrap());

                            println!("Deleting zip archive: {:?} after successfull parsing", main_path);
                            
                            match std::fs::remove_file(main_path).err() {
                                Some(e) => { println!("{:}", e); continue; },
                                None => continue,
                            };
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

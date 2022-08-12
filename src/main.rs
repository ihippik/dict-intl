#![allow(unused)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use clap::Parser;
use regex::Regex;
use log::{info, debug, warn, error, LevelFilter};
use serde::{Deserialize, Serialize};
use std::result::Result;
use walkdir::WalkDir;
use simple_logger::SimpleLogger;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// Output files name
    #[clap(default_value="dictionary.json", short, long, value_parser)]
    output: String,

    /// The path to the source directory to read
    #[clap(short, long, value_parser)]
    path: std::path::PathBuf,
}

/// IntlError represent common service Error.
type IntlError = Box<dyn std::error::Error>;

/// IntlResult alias for Result with IntlError.
type IntlResult<T> = Result<T, IntlError>;

/// Row is a single row in output dictionary file.
struct Row {
    id: String,
    default_msg: String,
}

/// COMPONENT_PATTERN helps choose lines this component name.
const COMPONENT_PATTERN: &str = "FormattedMessage";

/// REGEX is a regular expression with id and default message params.
const REGEX: &str = r#"<FormattedMessage id="([a-zA-Z.-_]+)" defaultMessage="([a-z A-Z.-]+)"#;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let args = Cli::parse();
    let mut items = Vec::new();

    for entry in WalkDir::new(args.path) {
        let path = match entry{
            Ok(p) => p,
            Err(error) => {
                error!("source directory not found");
                return;
            }
        };

        if !path.path().is_dir(){
            let mut arr = match process_file(PathBuf::from(path.path())){
                Ok(arr) => arr,
                Err(error) => {
                    error!("could`t process file: {:?}", error);
                    continue;
                }
            };

            items.append(&mut arr);
        }
    }

    match process_items(items,&args.output){
        Ok(()) => (),
        Err(error) => error!("could`t process dictionary items: {}", error),
    }
}

/// process_file parse all react components with params to the array of Row structure.
fn process_file(file_path: std::path::PathBuf) -> IntlResult<Vec<Row>>{
    let content = std::fs::read_to_string(&file_path)?;
    let re = Regex::new(REGEX)?;
    let mut arr = Vec::new();

    for line in content.lines() {
        if line.contains(COMPONENT_PATTERN) {
            let txt = match re.captures(line) {
                Some(x) => x,
                None => continue
            };

            let id = txt.get(1).map_or("", |m| m.as_str());
            let default_msg = txt.get(2).map_or("", |m| m.as_str());
            let row = Row{id:id.to_string(),default_msg:default_msg.to_string()};

            arr.push(row);
        }
    }

    debug!("file path: {}, items:{}", file_path.display(), arr.len());

    return Ok(arr);
}

// process_items create json file from rows data.
fn process_items(mut items: Vec<Row>, file_name: &str) -> IntlResult<()> {
    let mut map = HashMap::new();

    for val in items.iter() {
        map.insert(&val.id, &val.default_msg);
    }

    let json = serde_json::to_string_pretty(&map)?;

    let path = Path::new(file_name);
    let mut file = match File::create(&path) {
        Err(e) => return Err(IntlError::from(e)),
        Ok(file) => file,
    };

    file.write_all(json.as_bytes())?;

    info!("dictionary was created, number of items {}", items.len());

    return Ok(());
}
//! `calajson` converts any spreadsheet supported by calamine into a stream of
//! json lines.

extern crate calamine;
extern crate clap;
extern crate failure;
extern crate serde_json;

use std::process;

use calamine::Reader;
use clap::{App, Arg};
use failure::Error;
use serde_json::json;

mod output;
use output::{Sheet, Row};

fn main() {
    let matches = App::new("calajson")
        .version("0.1.0")
        .about("Reads your workbook and outputs a stream of JSON lines.")
        .arg(Arg::with_name("WORKBOOK")
             .help("The spreadsheet to be converted (formats supported by calamine)")
             .required(true)
             .index(1))
        .get_matches();

    let input_path = matches.value_of("WORKBOOK").unwrap();

    if let Err(e) = run(input_path) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

fn run(input_path: &str) -> Result<(), Error> {
    let mut workbook = calamine::open_workbook_auto(input_path)?;

    println!("{}", json!({
        "type": "meta",
        "version": "0",
    }).to_string());

    let sheets = workbook.sheet_names().to_owned();

    for (si, s) in sheets.into_iter().enumerate() {
        let sheet = Sheet::new(si as u32, s.to_string());

        println!("{}", sheet.to_json());

        let range = workbook.worksheet_range(&s).unwrap()?;
        for (ri, r) in range.rows().enumerate() {
            println!("{}", Row::new(ri as u32, &sheet, r).to_json());
        }
    }


    Ok(())
}

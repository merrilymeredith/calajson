//! # calajson
//!
//! `calajson` converts any spreadsheet supported by calamine into a stream of
//! json lines.
//!
//! # Examples
//!
//! ```bash
//! calajson --help
//! calajson workbook.xlsx
//!
//! # List sheet names
//! calajson workbook.xlsx | jq 'select(.type == "sheet") | .name'
//!
//! # Dump all sheet 0 contents as CSV
//! calajson workbook.xlsx | jq -r 'select(.type == "row" and .sheet == 0) | .data | @csv'
//! ```
//!
//! # Description
//!
//! This tiny tool leverages [calamine][] to produce structured dumps of
//! spreadsheet files.  In many scripting languages, large XML spreadsheets are
//! onerous to load and very few libraries support streaming that data, so
//! a large spreadsheet requires enormous time and memory just to _begin_
//! working with it, not to mention doing something productive with it.
//!
//! calajson lets you open a process and read its lines in order to read
//! a spreadsheet, but you can also use [jq][] for other powerful manipulations,
//! as in the second example above which gets you a CSV dump of all sheets.
//!
//! [calamine]: https://crates.io/crates/calamine
//! [jq]: https://stedolan.github.io/jq/
//!
//! # Output Format
//!
//! All output to STDOUT is a stream of JSON objects, separated by newlines.
//! Key order is not guaranteed.
//!
//! The first line output is always a "meta" object, which must have a "version"
//! key.  The current version documented here is 0.
//!
//! When a new sheet begins, a "sheet" object is emitted, which must have an
//! "idx" key and a "name" key -- though the name may be an empty string.
//!
//! The bulk of the output, then, is "row" objects.  Rows must have a "sheet"
//! key, referencing the "idx" of a sheet, an "idx" key of their own,
//! representing their position in that sheet, and a "data" key, containing an
//! array of values from the spreadsheet row itself.
//!
//! Within "data", empty cells are represented as JSON's null.  All other values
//! are stringified via the Display trait from calamine's internal types, found
//! in the `DataType` enum.  There may be changes in the future with regard to
//! actual values vs spreadsheet formatting, and representing those actual
//! values correctly in JSON output.
//!
//! ```json
//! {"type":"meta","version":"0"}
//! {"idx":0,"name":"Sheet 1","type":"sheet"}
//! {"data":["Part Number","Description","Price"],"idx":0,"sheet":0,"type":"row"}
//! {"data":["1328B","Frob","75.99"],"idx":1,"sheet":0,"type":"row"}
//! ```
//!

use std::process;

use calamine::Reader;
use clap::{App, Arg};
use failure::Error;

mod output;
use crate::output::*;

fn main() {
    let matches = App::new("calajson")
        .version("0.1.0")
        .about("Reads your workbook and outputs a stream of JSON lines.")
        .arg(
            Arg::with_name("WORKBOOK")
                .help("The spreadsheet to be converted (formats supported by calamine)")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_path = matches.value_of("WORKBOOK").unwrap();

    if let Err(e) = run(input_path) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

fn run(input_path: &str) -> Result<(), Error> {
    let mut workbook = calamine::open_workbook_auto(input_path)?;

    Meta { version: 0 }.print_json();

    let sheets = workbook.sheet_names().to_owned();

    for (si, s) in sheets.into_iter().enumerate() {
        let sheet = Sheet::new(si as u32, &s);

        sheet.print_json();

        let range = workbook.worksheet_range(&s).unwrap()?;
        for (ri, r) in range.rows().enumerate() {
            Row::new(ri as u32, &sheet, r).print_json();
        }
    }

    Ok(())
}

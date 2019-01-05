use calamine::DataType;
use serde_json::{json, Value};

pub trait Json {
    fn to_json(&self) -> String;
    fn print_json(&self) {
        println!("{}", self.to_json());
    }
}

/// Represents the metadata header in output
pub struct Meta {
    pub version: u32,
}

impl Json for Meta {
    fn to_json(&self) -> String {
        json!({
            "type": "meta",
            "version": self.version,
        })
        .to_string()
    }
}

/// Represents a worksheet in output, declaring its name and index
pub struct Sheet {
    pub idx: u32,
    pub name: String,
}

impl Sheet {
    pub fn new(idx: u32, name: &str) -> Self {
        Sheet {
            idx,
            name: name.into(),
        }
    }
}

impl Json for Sheet {
    fn to_json(&self) -> String {
        json!({
            "type": "sheet",
            "idx": self.idx,
            "name": self.name,
        })
        .to_string()
    }
}

/// Represents a row in output, which sheet it's from, its index on that sheet, and its data
pub struct Row<'a> {
    pub idx: u32,
    pub sheet: &'a Sheet,
    pub data: &'a [DataType],
}

impl<'a> Row<'a> {
    pub fn new(idx: u32, sheet: &'a Sheet, data: &'a [DataType]) -> Self {
        Row { idx, sheet, data }
    }
}

impl<'a> Json for Row<'a> {
    fn to_json(&self) -> String {
        let row: Vec<_> = self
            .data
            .iter()
            .map(|c| match c {
                DataType::Empty => Value::Null,
                any => Value::String(format!("{}", any)),
            })
            .collect();

        json!({
            "type": "row",
            "idx": self.idx,
            "sheet": self.sheet.idx,
            "data": row,
        })
        .to_string()
    }
}

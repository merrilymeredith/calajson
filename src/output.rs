use calamine::DataType;
use serde_json::{json, Value};

// {"type": "sheet", "idx": 1, "name": ""}
pub struct Sheet {
    pub idx: u32,
    pub name: String,
}

impl Sheet {
    pub fn new(idx: u32, name: String) -> Self {
        Sheet { idx, name }
    }

    pub fn to_json(&self) -> String {
        json!({
            "type": "sheet",
            "idx": self.idx,
            "name": self.name,
        })
        .to_string()
    }
}

// {"type": "row", "idx": 1, "sheet": 1, "data": [...]}
pub struct Row<'a> {
    pub idx: u32,
    pub sheet: &'a Sheet,
    pub data: &'a [DataType],
}

impl<'a> Row<'a> {
    pub fn new(idx: u32, sheet: &'a Sheet, data: &'a [DataType]) -> Self {
        Row { idx, sheet, data }
    }

    pub fn to_json(&self) -> String {
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

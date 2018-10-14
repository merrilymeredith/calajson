
// {"type": "sheet", "idx": 1, "name": ""}
pub struct Sheet {
	pub idx: u32,
	pub name: String,
}

impl Sheet {
	pub fn new(idx: u32, name: String) -> Self {
		Sheet {idx, name}
	}
	
	pub fn to_json(&self) -> String {
		json!({
			"type": "sheet",
			"idx": self.idx,
			"name": self.name,
		}).to_string()
	}
}

// {"type": "row", "idx": 1, "sheet": 1, "data": [...]}
pub struct Row<'a> {
	pub idx: u32,
	pub sheet: &'a Sheet,
	pub data: Vec<String>,
}

impl<'a> Row<'a> {
	pub fn new(idx: u32, sheet: &'a Sheet, data: Vec<String>) -> Self {
		Row {idx, sheet, data}
	}
	
	pub fn to_json(&self) -> String {
		json!({
			"type": "row",
			"idx": self.idx,
			"sheet": self.sheet.idx,
			"data": self.data,
		}).to_string()
	}
}


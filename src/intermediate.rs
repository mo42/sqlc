use std::collections::{HashMap, HashSet};
use std::fs::File;

use std::io::Result;

use csv::Reader;

fn read_csv_columns(file_path: &str) -> Result<HashMap<String, String>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);
    let hdrs = rdr.headers()?.clone();
    let columns: Vec<_> = hdrs.iter().map(|s| s.to_string()).collect();
    let mut col_types: HashMap<String, String> = HashMap::new();
    for c in columns.iter() {
        col_types.insert(
            c.split(':').nth(0).unwrap().to_string(),
            c.split(':')
                .nth(2)
                .unwrap_or("string")
                .replace("<", "")
                .replace(">", "")
                .replace("string", "std::string"),
        );
    }
    Ok(col_types)
}

pub struct IntRep {
    pub from: Option<String>,
    pub selection: Vec<String>,
    pub filter: Vec<String>,
    pub filter_cols: HashSet<String>,
}

impl IntRep {
    pub fn new() -> Self {
        IntRep {
            from: None,
            selection: Vec::new(),
            filter: Vec::new(),
            filter_cols: HashSet::new(),
        }
    }
}

pub struct IntRepSchema {
    pub from: String,
    pub selection: Vec<String>,
    pub filter: Vec<String>,
    pub filter_cols: HashSet<String>,
    pub index_type: String,
    pub col_types: HashMap<String, String>,
}

impl IntRepSchema {
    pub fn new(ir: IntRep) -> Self {
        let from = &ir.from.clone().unwrap();
        let col_types = read_csv_columns(from).unwrap();
        let idx_type = col_types.get("INDEX").unwrap().to_string();
        IntRepSchema {
            from: from.to_string(),
            selection: ir.selection,
            filter: ir.filter,
            filter_cols: ir.filter_cols,
            index_type: idx_type,
            col_types: col_types,
        }
    }
}

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

#[derive(Debug)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub struct ColumnOrder {
    pub column: String,
    pub direction: OrderDirection,
}

#[derive(Debug)]
pub enum JoinOperator {
    Inner,
    Left,
    Right,
}

pub struct Join {
    pub source: String,
    pub operator: JoinOperator,
    pub constraint: String,
}

#[derive(Debug)]
pub struct WithAlias {
    pub expr: String,
    pub alias: String,
}

#[derive(Debug)]
pub enum SelectItem {
    Unnamed(String),
    WithAlias(WithAlias),
}

pub struct IntRep {
    pub from: Option<String>,
    pub selection: Vec<SelectItem>,
    pub filter: Vec<String>,
    pub filter_cols: HashSet<String>,
    pub joins: Vec<Join>,
    pub order_by: Vec<ColumnOrder>,
    pub limit: Option<String>,
}

impl IntRep {
    pub fn new() -> Self {
        IntRep {
            from: None,
            selection: Vec::new(),
            filter: Vec::new(),
            filter_cols: HashSet::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            limit: None,
        }
    }
}

pub struct IntRepSchema {
    pub from: String,
    pub selection: Vec<SelectItem>,
    pub filter: Vec<String>,
    pub filter_cols: HashSet<String>,
    pub joins: Vec<Join>,
    pub order_by: Vec<ColumnOrder>,
    pub limit: Option<String>,
    pub index_type: String,
    pub col_types: HashMap<String, String>,
}

impl IntRepSchema {
    pub fn new(ir: IntRep) -> Self {
        let from = &ir.from.clone().unwrap();
        let mut col_types = read_csv_columns(from).unwrap();
        for join in &ir.joins {
            col_types.extend(read_csv_columns(&join.source).unwrap());
        }
        for select_item in &ir.selection {
            match select_item {
                SelectItem::WithAlias(wa) => {
                    // TODO correctly infer column type
                    col_types.insert(
                        wa.alias.clone(),
                        col_types.get(&wa.expr).unwrap().to_string(),
                    );
                }
                _ => {}
            }
        }
        let idx_type = col_types.get("INDEX").unwrap().to_string();
        IntRepSchema {
            from: from.to_string(),
            selection: ir.selection,
            filter: ir.filter,
            filter_cols: ir.filter_cols,
            joins: ir.joins,
            order_by: ir.order_by,
            limit: ir.limit,
            index_type: idx_type,
            col_types: col_types,
        }
    }
}

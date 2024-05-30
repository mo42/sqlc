use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;

pub struct Schema {
    pub index_type: String,
    pub col_types: HashMap<String, String>,
}

fn get_column_names(filename: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = Reader::from_reader(file);
    let headers = reader.headers()?.clone();
    let column_names = headers.iter().map(|s| s.to_string()).collect();
    Ok(column_names)
}

pub fn generate_code(from: &String, selection: &Vec<String>, filter: &Vec<String>) {
    // COLUMN TYPES
    println!("#include <DataFrame/DataFrame.h>\n\n#include <iostream>");
    println!("using namespace hmdf;");
    // TODO analyse source file here to determine all types
    let mut col_types = HashMap::new();
    col_types.insert("ca".to_string(), "double".to_string());
    col_types.insert("cb".to_string(), "double".to_string());
    col_types.insert("cc".to_string(), "double".to_string());
    col_types.insert("cd".to_string(), "double".to_string());
    col_types.insert("ce".to_string(), "double".to_string());
    col_types.insert("cf".to_string(), "long".to_string());
    let schema = Schema {
        index_type: "std::string".to_string(),
        col_types: col_types,
    };
    println!("typedef {idx_t} idx_t;", idx_t=schema.index_type);
    println!("using SqlcDataFrame = StdDataFrame<idx_t>;");
    println!("int main(int, char**) {{");
    println!("  SqlcDataFrame df;");
    println!("  df.read(\"{file_name}\", io_format::csv2);", file_name=from);
    println!("  return 0;");
    println!("}}");
}

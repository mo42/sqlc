use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet};

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

pub fn generate_code(from: &String, selection: &Vec<String>, filter: &Vec<String>, filter_cols: &HashSet<String>) {
    println!("#include <DataFrame/DataFrame.h>\n\n#include <iostream>");
    println!("using namespace hmdf;");
    // TODO analyse source file here to determine all types
    let mut col_types = HashMap::new();
    col_types.insert("ca".to_string(), "double".to_string());
    col_types.insert("cb".to_string(), "double".to_string());
    col_types.insert("cc".to_string(), "int".to_string());
    let schema = Schema {
        index_type: "std::string".to_string(),
        col_types: col_types,
    };
    println!("typedef {idx_t} idx_t;", idx_t=schema.index_type);
    println!("using SqlcDataFrame = StdDataFrame<idx_t>;");
    println!("int main(int, char**) {{");
    println!("  SqlcDataFrame df;");
    println!("  df.read(\"{file_name}\", io_format::csv2);", file_name=from);
    print!("  auto where_functor = [](const idx_t&");
    for col in filter_cols.iter() {
        print!(", const {col_t} &{col}", col_t=schema.col_types.get(col).unwrap(), col=col);
    }
    println!(") -> bool {{");
    print!("    return ");
    for filter_token in filter.iter() {
        print!("{filter_token}", filter_token=filter_token);
    }
    println!(";");
    println!("  }};");
    println!("  auto where_df =");
    print!("    df.get_data_by_sel<");
    for col in filter_cols.iter() {
        print!("{col_t}, ", col_t=schema.col_types.get(col).unwrap());
    }
    print!("decltype(where_functor)");
    let distinct_col_types: HashSet<_> = schema.col_types.values().cloned().collect();
    for col_type in distinct_col_types {
        print!(", {col_t}", col_t=col_type);
    }
    print!(">(");
    for col in filter_cols.iter() {
        print!("\"{col}\", ", col=col);
    }
    println!("where_functor);");
    println!("  std::vector<idx_t> idx = where_df.get_index();");
    for col in selection.iter() {
        println!("  std::vector<{col_t}> {col} = where_df.get_column<{col_t}> (\"{col}\");",
                col_t=schema.col_types.get(col).unwrap(), col=col);
    }
    println!("  SqlcDataFrame select;");
    println!("  select.load_index(std::move(idx));");
    for col in selection.iter() {
        println!("  select.load_column(\"{col}\", std::move({col}));", col=col);
    }
    println!("  std::cout << select.to_string<double>() << std::endl;");
    println!("  return 0;");
    println!("}}");
}

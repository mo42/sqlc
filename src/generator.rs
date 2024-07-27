use crate::intermediate::IntRepSchema;
use std::collections::HashSet;

pub fn generate_code(ir: &IntRepSchema) {
    println!("#include <DataFrame/DataFrame.h>\n\n#include <iostream>");
    println!("using namespace hmdf;");
    println!("typedef {idx_t} idx_t;", idx_t = ir.index_type);
    println!("using SqlcDataFrame = StdDataFrame<idx_t>;");
    println!("int main(int, char**) {{");
    println!("  SqlcDataFrame df;");
    println!(
        "  df.read(\"{file_name}\", io_format::csv2);",
        file_name = ir.from
    );
    print!("  auto where_functor = [](const idx_t&");
    for col in ir.filter_cols.iter() {
        print!(
            ", const {col_t} &{col}",
            col_t = ir.col_types.get(col).unwrap(),
            col = col
        );
    }
    println!(") -> bool {{");
    print!("    return ");
    for filter_token in ir.filter.iter() {
        print!("{filter_token}", filter_token = filter_token);
    }
    println!(";");
    println!("  }};");
    println!("  auto where_df =");
    print!("    df.get_data_by_sel<");
    for col in ir.filter_cols.iter() {
        print!("{col_t}, ", col_t = ir.col_types.get(col).unwrap());
    }
    print!("decltype(where_functor)");
    let distinct_col_types: HashSet<_> = ir.col_types.values().cloned().collect();
    for col_type in distinct_col_types {
        print!(", {col_t}", col_t = col_type);
    }
    print!(">(");
    for col in ir.filter_cols.iter() {
        print!("\"{col}\", ", col = col);
    }
    println!("where_functor);");
    println!("  std::vector<idx_t> idx = where_df.get_index();");
    for col in ir.selection.iter() {
        println!(
            "  std::vector<{col_t}> {col} = where_df.get_column<{col_t}> (\"{col}\");",
            col_t = ir.col_types.get(col).unwrap(),
            col = col
        );
    }
    println!("  SqlcDataFrame select;");
    println!("  select.load_index(std::move(idx));");
    for col in ir.selection.iter() {
        println!(
            "  select.load_column(\"{col}\", std::move({col}));",
            col = col
        );
    }
    println!("  std::cout << select.to_string<double>() << std::endl;");
    println!("  return 0;");
    println!("}}");
}

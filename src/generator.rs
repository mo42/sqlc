use crate::intermediate::*;
use core::fmt;
use std::collections::HashSet;

impl fmt::Display for JoinOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let jop = match self {
            JoinOperator::Inner => "inner_join",
            JoinOperator::Left => "left_join",
            JoinOperator::Right => "right_join",
        };
        write!(f, "{}", jop)
    }
}

pub fn generate_code(ir: &IntRepSchema) {
    // HEADER
    println!("#include <DataFrame/DataFrame.h>\n\n#include <iostream>");
    println!("using namespace hmdf;");
    println!("typedef {idx_t} idx_t;", idx_t = ir.index_type);
    println!("using SqlcDataFrame = StdDataFrame<idx_t>;");
    println!("int main(int, char**) {{");
    // FROM and JOIN
    println!("  SqlcDataFrame df_main;");
    println!(
        "  df_main.read(\"{file_name}\", io_format::csv2);",
        file_name = ir.from
    );
    for (i, join) in ir.joins.iter().enumerate() {
        println!("  SqlcDataFrame df_join{i};", i = i);
        println!(
            "  df_join{i}.read(\"{file_name}\", io_format::csv2);",
            i = i,
            file_name = join.source
        );
    }
    print!("  SqlcDataFrame df = df_main");
    let distinct_col_types: HashSet<_> = ir.col_types.values().cloned().collect();
    for (i, join) in ir.joins.iter().enumerate() {
        println!(".join_by_column");
        println!("    <");
        println!("      decltype(df_join{i}),", i = i);
        print!("      {t}", t = ir.col_types.get(&join.constraint).unwrap());
        for col_type in &distinct_col_types {
            print!(",\n      {col_type}");
        }
        println!(
            "\n    >(df_join{i}, \"{0}\", hmdf::join_policy::{1});",
            join.constraint, join.operator
        );
    }
    // WHERE
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
    for col_type in &distinct_col_types {
        print!(", {col_t}", col_t = col_type);
    }
    print!(">(");
    for col in ir.filter_cols.iter() {
        print!("\"{col}\", ", col = col);
    }
    println!("where_functor);");

    // SELECT
    println!("  std::vector<idx_t> idx = where_df.get_index();");
    for select_item in ir.selection.iter() {
        match select_item {
            SelectItem::Unnamed(s) => {
                println!(
                    "  std::vector<{col_t}> {col} = where_df.get_column<{col_t}> (\"{col}\");",
                    col_t = ir.col_types.get(s).unwrap(),
                    col = s
                );
            }
            SelectItem::WithAlias(wa) => {
                println!(
                    "  std::vector<{col_t}> {alias} = where_df.get_column<{col_t}> (\"{col}\");",
                    col_t = ir.col_types.get(&wa.alias).unwrap(),
                    col = wa.expr,
                    alias = wa.alias,
                );
            }
        }
    }
    println!("  SqlcDataFrame select;");
    println!("  select.load_index(std::move(idx));");
    for select_item in ir.selection.iter() {
        match select_item {
            SelectItem::Unnamed(s) => {
                println!(
                    "  select.load_column(\"{col}\", std::move({col}));",
                    col = s
                );
            }
            SelectItem::WithAlias(wa) => {
                println!(
                    "  select.load_column(\"{col}\", std::move({col}));",
                    col = wa.alias
                );
            }
        }
    }

    // ORDER BY

    if ir.order_by.len() > 0 {
        print!("  select.sort<");
        for ob in &ir.order_by {
            print!("{}, ", ir.col_types.get(&ob.column).unwrap());
        }
        let col_types = &distinct_col_types
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .join(", ");
        print!("{}", col_types);
        let order_by = ir
            .order_by
            .iter()
            .map(|ob| match ob.direction {
                OrderDirection::Ascending => format!("\"{}\", sort_spec::ascen", ob.column),
                OrderDirection::Descending => format!("\"{}\", sort_spec::desce", ob.column),
            })
            .collect::<Vec<String>>()
            .join(", ");
        print!(">({});\n", order_by);
    }

    // END ORDER BY

    print!("  select.write<std::ostream");
    let mut distinct_select_col_t: HashSet<String> = HashSet::new();
    for select_item in ir.selection.iter() {
        match select_item {
            SelectItem::Unnamed(s) => {
                distinct_select_col_t.insert(ir.col_types.get(s).unwrap().to_string());
            }
            SelectItem::WithAlias(wa) => {
                // TODO rename
                distinct_select_col_t.insert(ir.col_types.get(&wa.alias).unwrap().to_string());
            }
        }
    }
    for col_t in distinct_select_col_t.iter() {
        print!(", {col_t}");
    }
    println!(">(std::cout, hmdf::io_format::csv, 5, false, 100);");
    println!("  return 0;");
    println!("}}");
}

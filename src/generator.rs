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

macro_rules! format_push {
    ($vec:expr, $fmt:expr) => {
        $vec.push(format!($fmt));
    };
    ($vec:expr, $fmt:expr $(, $arg:expr)*) => {
        $vec.push(format!($fmt $(, $arg)*));
    };
}

pub fn generate_code(ir: &IntRepSchema) -> String {
    let mut code: Vec<String> = vec![];
    // HEADER
    format_push!(
        code,
        "#include <DataFrame/DataFrame.h>\n#include <iostream>"
    );
    format_push!(code, "using namespace hmdf;");
    format_push!(code, "typedef {} idx_t;", ir.index_type);
    format_push!(code, "using SqlcDataFrame = StdDataFrame<idx_t>;");
    format_push!(code, "int main(int, char**) {{");
    // FROM and JOIN
    format_push!(code, "SqlcDataFrame df_main;");
    format_push!(code, "df_main.read(\"{}\", io_format::csv2);", ir.from);
    for (i, join) in ir.joins.iter().enumerate() {
        format_push!(code, "SqlcDataFrame df_join{};", i);
        format_push!(
            code,
            "df_join{}.read(\"{}\", io_format::csv2);",
            i,
            join.source
        );
    }
    format_push!(code, "SqlcDataFrame df = df_main");
    let distinct_col_types: HashSet<_> = ir.col_types.values().cloned().collect();
    for (i, join) in ir.joins.iter().enumerate() {
        format_push!(code, ".join_by_column");
        format_push!(code, "<decltype(df_join{})", i);
        format_push!(code, ",{}", ir.col_types.get(&join.constraint).unwrap());
        for col_type in &distinct_col_types {
            format_push!(code, ",{}", col_type);
        }
        format_push!(
            code,
            ">(df_join{}, \"{}\", hmdf::join_policy::{});",
            i,
            join.constraint,
            join.operator
        );
    }
    // WHERE
    format_push!(code, "auto where_functor = [](const idx_t&");
    for col in ir.filter_cols.iter() {
        format_push!(code, ", const {} &{}", ir.col_types.get(col).unwrap(), col);
    }
    format_push!(code, ") -> bool {{");
    format_push!(code, "return");
    for filter_token in ir.filter.iter() {
        format_push!(code, "{}", filter_token);
    }
    format_push!(code, ";");
    format_push!(code, "}};");
    format_push!(code, "auto where_df = df.get_data_by_sel<");
    for col in ir.filter_cols.iter() {
        format_push!(code, "{}, ", ir.col_types.get(col).unwrap());
    }
    format_push!(code, "decltype(where_functor)");
    for col_type in &distinct_col_types {
        format_push!(code, ",{}", col_type);
    }
    format_push!(code, ">(");
    for col in ir.filter_cols.iter() {
        format_push!(code, "\"{}\", ", col);
    }
    format_push!(code, "where_functor);");

    // SELECT
    format_push!(code, "std::vector<idx_t> idx = where_df.get_index();");
    for select_item in ir.selection.iter() {
        match select_item {
            SelectItem::Unnamed(s) => {
                format_push!(
                    code,
                    "std::vector<{}> {} = where_df.get_column<{}> (\"{}\");",
                    ir.col_types.get(s).unwrap(),
                    s,
                    ir.col_types.get(s).unwrap(),
                    s
                );
            }
            SelectItem::WithAlias(wa) => {
                format_push!(
                    code,
                    "std::vector<{}> {} = where_df.get_column<{}> (\"{}\");",
                    ir.col_types.get(&wa.alias).unwrap(),
                    wa.alias,
                    ir.col_types.get(&wa.alias).unwrap(),
                    wa.expr
                );
            }
        }
    }
    format_push!(code, "SqlcDataFrame select;");
    format_push!(code, "select.load_index(std::move(idx));");
    for select_item in ir.selection.iter() {
        match select_item {
            SelectItem::Unnamed(s) => {
                format_push!(code, "select.load_column(\"{}\", std::move({}));", s, s);
            }
            SelectItem::WithAlias(wa) => {
                format_push!(
                    code,
                    "select.load_column(\"{}\", std::move({}));",
                    wa.alias,
                    wa.alias
                );
            }
        }
    }

    // ORDER BY

    if ir.order_by.len() > 0 {
        format_push!(code, "select.sort<");
        for ob in &ir.order_by {
            format_push!(code, "{}, ", ir.col_types.get(&ob.column).unwrap());
        }
        let col_types = &distinct_col_types
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .join(", ");
        format_push!(code, "{}", col_types);
        let order_by = ir
            .order_by
            .iter()
            .map(|ob| match ob.direction {
                OrderDirection::Ascending => format!("\"{}\", sort_spec::ascen", ob.column),
                OrderDirection::Descending => format!("\"{}\", sort_spec::desce", ob.column),
            })
            .collect::<Vec<String>>()
            .join(", ");
        format_push!(code, ">({});\n", order_by);
    }

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
    // LIMIT
    if let Some(limit) = &ir.limit {
        format_push!(
            code,
            "  auto limited = select.get_top_n_data<{}>(\"INDEX\", {});",
            distinct_select_col_t
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .join(", "),
            limit
        );
    } else {
        format_push!(code, "auto limited = select;");
    }

    format_push!(code, "limited.write<std::ostream");
    for col_t in distinct_select_col_t.iter() {
        format_push!(code, ", {}", col_t);
    }
    format_push!(code, ">(std::cout, hmdf::io_format::csv, 5, false, 100);");
    format_push!(code, "  return 0;");
    format_push!(code, "}}");
    return code.join("\n");
}

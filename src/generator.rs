use crate::intermediate::*;
use core::fmt;
use std::collections::HashSet;

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
    format_push!(code, "#include \"algebra/csv_reader.hpp\"");
    format_push!(code, "#include \"algebra/operators.hpp\"");
    format_push!(code, "#include \"algebra/relation.hpp\"");
    format_push!(code, "using namespace algebra;");
    format_push!(code, "int main() {{");
    // Load first from
    format_push!(code, "Relation rel_main = load_csv(\"{}\");", ir.from);
    format_push!(
        code,
        "Relation selected = select(rel_main, [rel_main](const std::vector<Value>& row) {{ return "
    );
    for filter_token in ir.filter.iter() {
        format_push!(code, "{}", filter_token);
    }
    format_push!(code, ";}});");
    format_push!(code, "}}");
    return code.join("\n");
}

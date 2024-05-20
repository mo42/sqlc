use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

mod visitor;
use crate::visitor::{SqlVisitor, Visitor};

use csv::Reader;
use std::error::Error;
use std::fs::File;

fn get_column_names(filename: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = Reader::from_reader(file);
    let headers = reader.headers()?.clone();
    let column_names = headers.iter().map(|s| s.to_string()).collect();
    Ok(column_names)
}

fn query_sequence(from: &String, selection: &Vec<String>, filter: &Vec<String>) {
    // COLUMN TYPES
    println!("#include <iostream>\n#include <vector>\n#include \"rapidcsv.h\"\n");
    // TODO analyse source file here to determine all types
    let Ok(column_names) = get_column_names(&from) else {
        todo!();
    };
    for column_name in column_names.iter() {
        println!(
            "typedef std::string {column_name};",
            column_name = column_name
        );
    }
    println!("");
    println!("struct base {{");
    println!("  std::string _source = \"{from}\";", from = from);
    println!("  std::vector<std::string> _columns = {{");
    for column_name in column_names.iter() {
        println!("    \"{column_name}\",", column_name = column_name);
    }
    println!("  }};");
    for column_name in column_names.iter() {
        println!(
            "  std::vector<std::string> {column_name};",
            column_name = column_name
        );
    }
    println!("}};");
    println!("");
    println!("base load_base() {{");
    println!("  base table;");
    println!("  rapidcsv::Document doc(table._source);");
    for column_name in column_names.iter() {
        println!(
            "  table.{column_name} = doc.GetColumn<std::string>(\"{column_name}\");",
            column_name = column_name
        );
    }
    println!("  return table;");
    println!("}}");
    println!("");

    println!("base filter(base table) {{");
    println!("  base out;");
    println!("  for (std::size_t i = 0; i != table.ca.size(); ++i) {{");
    println!("    if ({filter}) {{", filter=filter.join(" "));
    for column_name in column_names.iter() {
        println!("      out.{column_name}.push_back(table.ca[i]);");
    }
    println!("    }}");
    println!("  }}");
    println!("return out;");
    println!("}}");
    println!("");

    println!("struct projected {{");
    println!("  std::vector<std::string> _columns = {{");
    for column_name in selection.iter() {
        println!("    \"{column_name}\",", column_name = column_name);
    }
    println!("  }};");
    for column_name in selection.iter() {
        println!(
            "  std::vector<std::string> {column_name};",
            column_name = column_name
        );
    }
    println!("}};");
    println!("");

    println!("projected project(base table) {{");
    println!("  projected result;");
    for column_name in selection.iter() {
        println!(
            "  result.{column_name} = table.{column_name};",
            column_name = column_name
        );
    }
    println!("  return result;");
    println!("}}");
    println!("");

    println!("int main() {{");
    println!("  projected table = project(filter(load_base()));");
    println!("  return 0;");
    println!("}}");
}

fn traverse_ast(visitor: &mut dyn Visitor, statement: &Statement) {
    visitor.visit_statement(statement);
}

fn main() {
    let sql = "SELECT cc FROM 'ta.csv' WHERE cb = 1 AND ca = 2";
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    dbg!("{:?}", &ast);
    let mut visitor = SqlVisitor::new();
    for statement in ast {
        traverse_ast(&mut visitor, &statement);
    }
    query_sequence(&visitor.from.unwrap(), &visitor.selection, &visitor.filter);
}

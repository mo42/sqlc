use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

mod visitor;
use crate::visitor::{traverse_ast, SqlVisitor};

mod generator;
use crate::generator::generate_code;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    } else {
        match fs::read_to_string(&args[1]) {
            Ok(read_file) => {
                let dialect = GenericDialect {};
                let ast = Parser::parse_sql(&dialect, &read_file).unwrap();

                let mut visitor = SqlVisitor::new();
                for statement in ast {
                    traverse_ast(&mut visitor, &statement);
                }
                generate_code(&visitor.from.unwrap(), &visitor.selection, &visitor.filter);
            }
            Err(error) => {
                eprintln!("Error reading file {}: {}", &args[1], error);
                process::exit(1);
            }
        }
    }
}

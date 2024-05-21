use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

mod visitor;
use crate::visitor::{SqlVisitor, traverse_ast};

mod generator;
use crate::generator::generate_code;


fn main() {
    let sql = "SELECT cc FROM 'ta.csv' WHERE cb = 1 AND ca = 2";
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    dbg!("{:?}", &ast);
    let mut visitor = SqlVisitor::new();
    for statement in ast {
        traverse_ast(&mut visitor, &statement);
    }
    generate_code(&visitor.from.unwrap(), &visitor.selection, &visitor.filter);
}

use sqlparser::ast::*;
use std::collections::HashSet;

pub trait Visitor {
    fn visit_statement(&mut self, statement: &Statement);
    fn visit_body(&mut self, body: &SetExpr);
    fn visit_select(&mut self, select: &Select);
    fn visit_select_item(&mut self, select_item: &SelectItem);
    fn visit_table_with_joins(&mut self, table_with_joins: &TableWithJoins);
    fn visit_expr(&mut self, expr: &Expr);
    fn visit_relation(&mut self, relation: &TableFactor);
    fn visit_object_name(&mut self, object_name: &ObjectName);
    fn visit_ident(&mut self, ident: &Ident) -> Option<String>;
    fn visit_value(&mut self, value: &Value) -> String;
    fn visit_query(&mut self, query: &Query);
    fn visit_expression(&mut self, expression: &Expr) -> Vec<String>;
    fn visit_operator(&mut self, op: &BinaryOperator) -> String;
}

pub struct SqlVisitor {
    pub from: Option<String>,
    pub selection: Vec<String>,
    pub filter: Vec<String>,
    pub filter_cols: HashSet<String>,
}

impl SqlVisitor {
    pub fn new() -> Self {
        SqlVisitor {
            from: None,
            selection: Vec::new(),
            filter: Vec::new(),
            filter_cols: HashSet::new(),
        }
    }
}

impl Visitor for SqlVisitor {
    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Query(query) => self.visit_query(query),
            _ => {}
        }
    }

    fn visit_query(&mut self, query: &Query) {
        self.visit_body(&query.body);
    }

    fn visit_body(&mut self, body: &SetExpr) {
        match body {
            SetExpr::Select(select) => {
                self.visit_select(select);
            }
            _ => {}
        }
    }

    fn visit_select(&mut self, select: &Select) {
        for select_item in &select.projection {
            self.visit_select_item(&select_item);
        }
        for table_with_joins in &select.from {
            self.visit_table_with_joins(&table_with_joins);
        }
        match &select.selection {
            Some(selection) => {
                self.filter = self.visit_expression(&selection);
            }
            None => {}
        }
    }

    fn visit_select_item(&mut self, select_item: &SelectItem) {
        match select_item {
            SelectItem::UnnamedExpr(expr) => {
                self.visit_expr(&expr);
            }
            _ => {}
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(ident) => {
                let col = self.visit_ident(&ident).unwrap().clone();
                self.selection.push(col);
            }
            _ => {}
        }
    }

    fn visit_table_with_joins(&mut self, table_with_joins: &TableWithJoins) {
        self.visit_relation(&table_with_joins.relation);
    }

    fn visit_relation(&mut self, relation: &TableFactor) {
        match relation {
            TableFactor::Table { name, .. } => {
                self.visit_object_name(&name);
            }
            _ => {}
        }
    }

    fn visit_object_name(&mut self, object_name: &ObjectName) {
        for ident in object_name.0.iter() {
            self.from = self.visit_ident(&ident);
        }
    }

    fn visit_ident(&mut self, ident: &Ident) -> Option<String> {
        return Some(ident.value.to_string());
    }

    fn visit_expression(&mut self, expression: &Expr) -> Vec<String> {
        match expression {
            Expr::BinaryOp { left, op, right } => {
                // TODO For now, directly translate to C expression
                let mut r = vec!["(".to_string()];
                r.extend(self.visit_expression(left));
                r.push(self.visit_operator(op));
                r.extend(self.visit_expression(right));
                r.push(")".to_string());
                return r;
            }
            Expr::Identifier(ident) => {
                // TODO For now, directly translate to C expression
                let id = self.visit_ident(&ident).unwrap();
                self.filter_cols.insert(id.clone());
                return [id.clone()].to_vec();
            }
            Expr::Value(val) => {
                let r: String = self.visit_value(&val).to_string();
                return [r].to_vec();
            }
            _ => [].to_vec(),
        }
    }

    fn visit_value(&mut self, value: &Value) -> String {
        match value {
            Value::Number(num, _) => {
                return num.to_string();
            }
            _ => {
                return String::new();
            }
        }
    }

    fn visit_operator(&mut self, op: &BinaryOperator) -> String {
        match op {
            BinaryOperator::Plus => "+".to_string(),
            BinaryOperator::Minus => "-".to_string(),
            BinaryOperator::Multiply => "*".to_string(),
            BinaryOperator::Divide => "/".to_string(),
            BinaryOperator::Modulo => "%".to_string(),
            BinaryOperator::Gt => ">".to_string(),
            BinaryOperator::Lt => "<".to_string(),
            BinaryOperator::GtEq => ">=".to_string(),
            BinaryOperator::LtEq => "<=".to_string(),
            BinaryOperator::Eq => "==".to_string(),
            BinaryOperator::NotEq => "!+".to_string(),
            BinaryOperator::And => "&&".to_string(),
            BinaryOperator::Or => "||".to_string(),
            _ => {
                todo!();
            }
        }
    }
}

pub fn traverse_ast(visitor: &mut dyn Visitor, statement: &Statement) {
    visitor.visit_statement(statement);
}

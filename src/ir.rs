use sqlparser::ast::*;
use std::collections::HashSet;

pub struct SqlVisitor {
    pub from: Option<String>,
    pub selection: Vec<String>,
    pub filter: Vec<String>,
    pub filter_cols: HashSet<String>,
    pub group_by: Vec<String>,
}

impl SqlVisitor {
    pub fn new() -> Self {
        SqlVisitor {
            from: None,
            selection: Vec::new(),
            filter: Vec::new(),
            filter_cols: HashSet::new(),
            group_by: Vec::new(),
        }
    }
}

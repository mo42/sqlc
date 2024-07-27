use std::collections::HashSet;

pub struct IntRep {
    pub from: Option<String>,
    pub selection: Vec<String>,
    pub filter: Vec<String>,
    pub filter_cols: HashSet<String>,
}

impl IntRep {
    pub fn new() -> Self {
        IntRep {
            from: None,
            selection: Vec::new(),
            filter: Vec::new(),
            filter_cols: HashSet::new(),
        }
    }
}

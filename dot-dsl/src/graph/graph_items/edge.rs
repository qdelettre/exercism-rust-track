use std::collections::HashMap;

use maplit::hashmap;

use crate::impl_with_attrs;

#[derive(Debug, Clone, PartialEq)]
pub struct Edge<'a> {
    ends: (&'a str, &'a str),
    attrs: HashMap<String, String>,
}

impl<'a> Edge<'a> {
    pub fn new(left: &'a str, right: &'a str) -> Self {
        Edge {
            ends: (left, right),
            attrs: hashmap! {},
        }
    }

    impl_with_attrs!(Edge);
}

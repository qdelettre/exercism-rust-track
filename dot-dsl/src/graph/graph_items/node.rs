use std::collections::HashMap;

use maplit::hashmap;

use crate::impl_with_attrs;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    pub name: &'a str,
    attrs: HashMap<String, String>,
}

impl<'a> Node<'a> {
    pub fn new(name: &'a str) -> Self {
        Node {
            name,
            attrs: hashmap! {},
        }
    }

    impl_with_attrs!(Node);
}

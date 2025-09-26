use crate::rule::utils::normalize;
use fmt_runner::pipeline::{EditTarget, StructuredPass};
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;
use tree_sitter::Node;

pub struct PackagePass<C> {
    _marker: PhantomData<C>,
}

impl<C> PackagePass<C> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<C> StructuredPass for PackagePass<C>
where
    C: Serialize + DeserializeOwned,
{
    type Config = C;
    type Item = String;

    fn find_targets(&self, root: &Node, source: &str) -> Vec<EditTarget<Self::Item>> {
        let mut cursor = root.walk();
        root.children(&mut cursor)
            .filter(|child| child.kind() == "package_declaration")
            .map(|child| {
                let range = (child.start_byte(), child.end_byte());
                EditTarget {
                    range,
                    items: vec![normalize(&source[range.0..range.1])],
                }
            })
            .collect()
    }

    fn build(&self, _config: &Self::Config, items: &[Self::Item]) -> String {
        items.join("\n")
    }
}

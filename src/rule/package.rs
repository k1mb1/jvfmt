use crate::rule::utils::normalize;
use fmt_runner::pipeline::{EditTarget, StructuredPass};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tree_sitter::Node;


pub struct PackagePass;

impl<Config> StructuredPass<Config> for PackagePass
where
    Config: Serialize + DeserializeOwned,
{
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

    fn build(&self, items: &[Self::Item]) -> String {
        items.join("\n")
    }
}

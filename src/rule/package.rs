use fmtrunner::pipeline::{EditTarget, StructuredPass};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tree_sitter::Node;


pub fn normalize(raw: &str) -> String {
    raw.split_whitespace().join(" ").replace(" ;", ";")
}

pub struct PackagePass;

pub struct Package {
    content: String,
}

impl Package {
    fn new(content: &str) -> Self {
        Self {
            content: normalize(content),
        }
    }
}

impl<C> StructuredPass<C> for PackagePass
where
    C: Serialize + for<'a> Deserialize<'a>,
{
    type Item = Package;

    fn find_targets(&self, root: &Node, source: &String) -> Vec<EditTarget<Self::Item>> {
        let mut targets = Vec::new();
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            if child.kind() == "package_declaration" {
                let range = (child.start_byte(), child.end_byte());
                let raw = &source[range.0..range.1];

                targets.push(EditTarget {
                    range,
                    items: vec![Package::new(raw)],
                });
            }
        }

        targets
    }

    fn build(&self, items: &[Self::Item]) -> String {
        items
            .iter()
            .map(|p| p.content.clone())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

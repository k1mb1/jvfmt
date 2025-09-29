use crate::config::Config;
use crate::rule::utils::normalize;
use fmt_runner::EditTarget;
use fmt_runner::StructuredPass;
use tree_sitter::Node;

pub struct PackagePass;

impl StructuredPass for PackagePass {
    type Config = Config;
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

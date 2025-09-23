use crate::config::ImportConfigProvider;
use crate::rule::utils::normalize;
use fmt_runner::pipeline::{EditTarget, StructuredPass};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::cmp::Ordering;
use tree_sitter::Node;


pub struct ImportsPass;

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    /// full (normalized) content including trailing semicolon
    content: String,
    /// group of import
    group: ImportGroup,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum ImportGroup {
    NonJava,
    JavaCore,
    Static,
}

impl Import {
    fn new(raw: &str) -> Self {
        let norm_content = normalize(raw);
        Self {
            content: norm_content.clone(),
            group: Self::group(&norm_content),
        }
    }

    fn group(content: &str) -> ImportGroup {
        if content.contains(" static ") {
            ImportGroup::Static
        } else if content.contains(" java.") {
            ImportGroup::JavaCore
        } else {
            ImportGroup::NonJava
        }
    }
}

impl PartialOrd for Import {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Import {
    fn cmp(&self, other: &Self) -> Ordering {
        self.group
            .cmp(&other.group)
            .then_with(|| self.content.cmp(&other.content))
    }
}

impl<C> StructuredPass<C> for ImportsPass
where
    C: Serialize + DeserializeOwned + ImportConfigProvider,
{
    type Item = Import;

    fn find_targets(&self, root: &Node, source: &str) -> Vec<EditTarget<Self::Item>> {
        let mut cursor = root.walk();
        let mut ranges = Vec::new();

        for child in root.children(&mut cursor) {
            if child.kind() == "import_declaration" {
                ranges.push((child.start_byte(), child.end_byte()));
            }
        }

        if ranges.is_empty() {
            return Vec::new();
        }

        vec![EditTarget {
            range: (ranges[0].0, ranges.last().unwrap().1),
            items: ranges
                .iter()
                .map(|(s, e)| Import::new(&source[*s..*e]))
                .collect(),
        }]
    }

    fn transform(
        &self,
        _root: &Node,
        _source: &str,
        config: &C,
        items: &mut Vec<Self::Item>,
    ) -> Result<(), String> {
        if config.import_config().sort {
            items.sort();
        }
        Ok(())
    }

    fn build(&self, imports: &[Self::Item]) -> String {
        let mut result = Vec::new();
        let mut current_group = imports.first().map(|imp| imp.group);

        for imp in imports {
            if let Some(group) = current_group {
                if imp.group != group {
                    result.push(String::new());
                    current_group = Some(imp.group);
                }
            }
            result.push(imp.content.clone());
        }

        result.join("\n")
    }
}

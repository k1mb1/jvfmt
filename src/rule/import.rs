use crate::config::{Config, ImportConfigProvider};
use crate::rule::utils::normalize;
use fmt_runner::{EditTarget, StructuredPass};
use std::cmp::Ordering;
use tree_sitter::Node;

pub struct ImportsPass;

#[derive(Debug)]
pub struct Import {
    content: String,
    group: ImportGroup,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum ImportGroup {
    NonJava,
    JavaCore,
    Static,
}

impl Import {
    fn new(raw: &str) -> Self {
        let content = normalize(raw);
        let group = if content.contains(" static ") {
            ImportGroup::Static
        } else if content.contains(" java.") {
            ImportGroup::JavaCore
        } else {
            ImportGroup::NonJava
        };
        Self { content, group }
    }
}

impl PartialEq for Import {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Eq for Import {}

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

impl StructuredPass for ImportsPass {
    type Config = Config;
    type Item = Import;

    fn extract(&self, root: &Node, source: &str) -> Vec<EditTarget<Self::Item>> {
        let mut cursor = root.walk();
        let ranges: Vec<_> = root
            .children(&mut cursor)
            .filter(|child| child.kind() == "import_declaration")
            .map(|child| (child.start_byte(), child.end_byte()))
            .collect();

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
        config: &Self::Config,
        items: &mut Vec<Self::Item>,
    ) -> Result<(), String> {
        if config.import_config().sort {
            items.sort();
        }
        Ok(())
    }

    fn build(&self, config: &Self::Config, imports: &[Self::Item]) -> String {
        let mut result = Vec::new();
        let mut prev_group = None;

        for imp in imports {
            if config.import_config().grouped
                && let Some(group) = prev_group
                && imp.group != group
            {
                result.push(String::new());
            }
            result.push(imp.content.clone());
            prev_group = Some(imp.group);
        }

        result.join("\n")
    }
}

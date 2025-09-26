use itertools::Itertools;

pub fn normalize(raw: &str) -> String {
    raw.split_whitespace().join(" ").replace(" ;", ";")
}

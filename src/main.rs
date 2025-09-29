mod config;
mod java_parser;
mod rule;

use crate::config::Config;
use crate::java_parser::Java;
use crate::rule::{ImportsPass, PackagePass};
use fmt_runner::cli_builder;

fn main() {
    cli_builder::<Java, Config>()
        .add_pass(PackagePass::new())
        .add_pass(ImportsPass::new())
        .run();
}

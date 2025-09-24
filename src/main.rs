mod config;
mod java_parser;
mod rule;


use crate::config::Config;
use crate::java_parser::Java;
use crate::rule::{ImportsPass, PackagePass};
use fmt_runner::cli::handle_cli;
use fmt_runner::pipeline::Pipeline;


fn main() {
    let mut pipeline = Pipeline::new();
    pipeline.add_pass(PackagePass::new());
    pipeline.add_pass(ImportsPass::new());

    handle_cli::<Java, Config>(pipeline);
}

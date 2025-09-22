mod config;
mod java_parser;
mod rule;


use crate::config::Config;
use crate::java_parser::Java;
use crate::rule::PackagePass;
use fmtrunner::cli::handle_cli;
use fmtrunner::pipeline::Pipeline;


fn main() {

    let mut pipeline = Pipeline::new();
    pipeline.add_pass(PackagePass);

    handle_cli::<Java, Config>(pipeline);
}

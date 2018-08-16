#![feature(custom_attribute)]

extern crate clap;
extern crate clap_verbosity_flag;
extern crate clap_log_flag;
#[macro_use]
extern crate structopt;

use clap::{App, SubCommand};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[structopt(flatten)]
    log: clap_log_flag::Log,
}

fn main() {
    let args = Cli::from_args();
}


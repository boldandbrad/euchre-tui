use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct Args {}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "An animated cat for your terminal (Maxwell, courtesy of ascii-live)", long_about = None)]
pub struct Cli {}

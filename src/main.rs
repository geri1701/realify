use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
#[derive(Parser)]
#[command(name = "Realify")]
#[command(author = "Gerhard S. <geri@sdf.org>")]
#[command(version = "0.1.0")]
#[command(about = "Assigns a name to a key.", long_about = None)]
struct Cli {
    #[arg(long)]
    config: String,
    #[arg(long)]
    name: String,
}
fn main() {
    let cli = Cli::parse();
    let mut config = File::open(cli.config).expect("Failed to open file");
    let mut table = HashMap::<String, String>::new();
    let mut contents = String::new();
    config
        .read_to_string(&mut contents)
        .expect("Failed to read file");
    for line in contents.lines() {
        if let Some((k, v)) = line.split_once(" ") {
            table.insert(k.into(), v.into());
        }
    }
}

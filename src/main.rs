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

fn realify(config: &str, name: &str) -> Result<String, String> {
    let mut config_file = File::open(config).map_err(|e| e.to_string())?;
    let mut table = HashMap::<String, String>::new();
    let mut contents = String::new();
    config_file
        .read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;
    for line in contents.lines() {
        if let Some((k, v)) = line.split_once(" ") {
            table.insert(k.into(), v.into());
        }
    }
    if let Some(value) = table.get(name) {
        Ok(value.to_string())
    } else {
        Err(format!(
            "Error: Your config doesn't contain the key {}",
            name
        ))
    }
}

fn main() {
    let cli = Cli::parse();
    match realify(&cli.config, &cli.name) {
        Ok(value) => println!("{}", value),
        Err(err) => eprintln!("{}", err),
    }
}

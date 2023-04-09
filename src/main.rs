use clap::Parser;
use std::collections::HashMap;
use std::fs;

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
fn read_config_file(config: &str) -> Result<HashMap<String, String>, String> {
    let contents = fs::read_to_string(config).map_err(|e| e.to_string())?;
    let mut table = HashMap::new();
    for line in contents.lines() {
        if let Some((k, v)) = line.split_once(' ') {
            table.insert(k.into(), v.into());
        }
    }
    Ok(table)
}

fn realify(table: &HashMap<String, String>, name: &str) -> Result<String, String> {
    table
        .get(name)
        .cloned()
        .ok_or_else(|| format!("Error: Your config doesn't contain the key {}", name))
}

fn main() {
    let cli = Cli::parse();
    match read_config_file(&cli.config).and_then(|table| realify(&table, &cli.name)) {
        Ok(value) => println!("{}", value),
        Err(err) => eprintln!("{}", err),
    }
}

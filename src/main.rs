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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_realify() {
        let mut table = HashMap::new();
        table.insert(String::from("foo"), String::from("bar"));

        let result = realify(&table, "foo");
        assert_eq!(result, Ok(String::from("bar")));

        let result = realify(&table, "baz");
        assert_eq!(
            result,
            Err(String::from(
                "Error: Your config doesn't contain the key baz"
            ))
        );
    }

    #[test]
    fn test_read_config_file() {
        let mut config_file = std::fs::File::create("test_config.txt").unwrap();
        let data = "key1 value1\nkey2 value2\nkey3 value3\n";
        config_file.write_all(data.as_bytes()).unwrap();
        let result = read_config_file("test_config.txt");
        let mut expected = std::collections::HashMap::new();
        expected.insert(String::from("key1"), String::from("value1"));
        expected.insert(String::from("key2"), String::from("value2"));
        expected.insert(String::from("key3"), String::from("value3"));
        assert_eq!(result, Ok(expected));
        // Cleanup
        std::fs::remove_file("test_config.txt").unwrap();
    }
}

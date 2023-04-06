use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut config = File::open("~/.realify.conf").expect("Failed to open file");
    let mut assign = HashMap::<String, String>::new();
    let mut contents = String::new();
    config
        .read_to_string(&mut contents)
        .expect("Failed to read file");
    for line in contents.lines() {
        if let Some((k, v)) = line.split_once(" ") {
            assign.insert(k.into(), v.into());
        }
    }
}

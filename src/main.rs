use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Read, Write},
};

use clap::Parser;

/// Simple program to convert ascii art into braille dot art
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Chat window length size
    #[arg(short, long)]
    length: usize,
    /// Pre-padding length
    #[arg(short, long, default_value_t = 0)]
    padding: usize,
}

fn get_char_to_braille_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let f = fs::File::open("mapping").unwrap();
    let rdr = BufReader::new(f);

    for line in rdr.lines() {
        let Ok(line) = line else {
            continue;
        };

        let mut chars = line.chars();
        let char = chars.next().unwrap();
        let braille = chars.next().unwrap();

        map.insert(char.to_string(), braille.to_string());
    }

    map
}

fn get_input_from_user() -> String {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    String::from_utf8(input).unwrap()
}

fn main() {
    let args = Args::parse();
    let map = get_char_to_braille_map();
    let input = get_input_from_user();
    let mut padded_input = String::from_utf8(vec![b' '; args.length]).unwrap();
    padded_input.push('\n');

    for line in input.lines() {
        for _ in 0..args.padding {
            padded_input.push(' ');
        }

        padded_input.push_str(line);

        for _ in 0..(args.length - line.len() - args.padding) {
            padded_input.push(' ');
        }
        padded_input.push('\n');
    }

    for (char, braille) in map {
        padded_input = padded_input.replace(&char, &braille);
    }

    std::io::stdout()
        .write_all(padded_input.as_bytes())
        .unwrap();
}

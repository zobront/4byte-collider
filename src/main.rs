use ethers::utils;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TARGET: &str = "INSERT_TARGET_SIGNATURE";

fn calculate_selector(signature: &str) -> [u8; 4] {
    utils::id(signature)
}

fn read_words(filename: &Path) -> io::Result<Vec<String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().filter_map(io::Result::ok).collect())
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() -> io::Result<()> {
    let target: [u8; 4] = calculate_selector(TARGET);

    let intros = read_words(Path::new("./words/intros.txt"))?;

    let nouns = read_words(Path::new("./words/nouns.txt"))?;
    let capitalized_nouns: Vec<String> = nouns.iter().map(|n|title_case(n)).collect();

    let verbs = read_words(Path::new("./words/verbs.txt"))?;
    let capitalized_verbs: Vec<String> = verbs.iter().map(|v| title_case(v)).collect();

    let types = read_words(Path::new("./words/types.txt"))?;
    let mut type_strings: Vec<String> = Vec::new();
    for a in types.iter() {
        for b in types.iter() {
            for c in types.iter() {
                for d in types.iter() {
                    let type_string = format!("{},{},{},{}", a, b, c, d);
                    type_strings.push(type_string);
                }
            }
        }
    }

    for intro in &intros {
        for noun in &capitalized_nouns {
            for verb in &capitalized_verbs {
                for types in &type_strings {
                    let signature = format!("{}{}Fees{}({})", intro, noun, verb, types);

                    if calculate_selector(&signature) == target {
                        println!("WINNER: {}", &signature);
                    }
                }
            }
        }
    }
    Ok(())
}

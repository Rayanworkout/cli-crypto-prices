use std::io;
use strsim::levenshtein;
use std::collections::HashMap;

pub fn capitalize(token: &str) -> String {
    // Function to capitalize a given string reference
    // If string reference is null, returns a new string
    // Note that the first letter does not appear twice
    // Because of the behavior of next(), which advance the iterator
    // So the first char is exhausted
    let mut char_1 = token.chars();

    match char_1.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + char_1.as_str(),
    }
}

pub fn collect_cli_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!(
            "\n> Please provide at least one token name (see https://www.coingecko.com/ for full list)."
        );
        collect_input_arg()
    } else {
        args[1..]
            .iter()
            .map(|str| str.to_lowercase())
            .collect::<Vec<String>>()
    }
}

pub fn collect_input_arg() -> Vec<String> {
    let mut input = String::new();

    // Allowing user to input token name if not provided at runtime
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    vec![input.trim().to_lowercase()]
}

pub fn confirm_choice() -> bool {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let yes = ["y".to_string(), "yes".to_string()].contains(&choice.trim().to_lowercase());

    if yes {
        true
    } else {
        false
    }
}


/// Function to find the closest match of user input
/// required to change one string into the other.
///
/// ```
/// use strsim::levenshtein;
///
/// assert_eq!(3, levenshtein("kitten", "sitting"));
/// ```
pub fn find_closest_match<'a>(input: &'a str, crypto_names: &'a Vec<String>) -> Option<&'a str> {
    let mut similarities: HashMap<&str, usize> = HashMap::new();

    for name in crypto_names {
        let distance = levenshtein(input, &name);
        similarities.insert(&name, distance);
    }

    let (closest_name, _) = similarities.iter().min_by_key(|&(_, &dist)| dist)?;

    Some(closest_name)
}
// use regex::Regex;

struct Password {
    minOcc: usize,
    maxOcc: usize,
    rule_letter: char,
    pass: String,
}

// fn analyze_slice(slice: &String) {}

fn main() {
    println!("Hello, world!");

    let word = "1-3 a: abcde";
    let parts: Vec<&str> = word
        .split(['-', ' ', ':'].as_ref())
        .filter(|s| !s.is_empty())
        .collect();
    println!("{:?}", parts);

    let pass = Password {
        minOcc: parts[0],
        maxOcc: parts[1],
        rule_letter: parts[2],
        pass: parts[3],
    };
}

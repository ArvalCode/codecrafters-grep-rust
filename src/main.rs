use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else if pattern == "\\d" {
        return input_line.contains(|c: char| c.is_digit(10));
    } else if pattern == "\\w" {
        return input_line.contains(|c: char | c.is_alphanumeric());
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        let mut new_pattern = pattern.trim_matches('[').trim_matches(']').bytes();
        return input_line.bytes().any(|val| new_pattern.any(|p| val == p))
    } else if pattern.starts_with("[^") && pattern.ends_with(']') {
        // Extract the characters excluded by the pattern
        let cuttern = &pattern[2..pattern.len() - 1];
        let result = !input_line.chars().any(|c| cuttern.contains(c));
        if result {
            return false;
        } else {
            return true;
        }
    }
     else {
        panic!("Unhandled pattern: {}", pattern)
    }
}


// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
     if match_pattern(&input_line, &pattern) {
         process::exit(0)
     } else {
         process::exit(1)
     }
}

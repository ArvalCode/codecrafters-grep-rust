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
        return input_line.bytes().any(|val| new_pattern.any(|p| val == p));
    } else if pattern.starts_with("[^") && pattern.ends_with(']') {
        if pattern.starts_with("[^") && pattern.ends_with(']') {
            let chars_to_exclude: String = pattern[2..pattern.len()-1].to_string();
            let exclude_chars: std::collections::HashSet<char> = chars_to_exclude.chars().collect();
            
            // Return true if none of the input line's characters are in the exclude set
            !input_line.chars().any(|c| exclude_chars.contains(&c))
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

use std::env;
use std::io;
use std::process;
use std::str::Chars;

#[derive(Debug, PartialEq)] // Derive PartialEq for pattern matching
enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
    StartOfLine,
}

fn match_literal(chars: &mut Chars, literal: char) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c == literal)
}

fn match_digit(chars: &mut Chars) -> bool {
    let c = chars.next();
    if c.is_none() {
        return false;
    }
    c.unwrap().is_digit(10)
}

fn match_alphanumeric(chars: &mut Chars) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c.is_alphanumeric())
}

fn match_group(chars: &mut Chars, group: &str) -> bool {
    let c = chars.next();
    c.is_some_and(|c| group.contains(c))
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let patterns = build_patterns(pattern);
    let input_line = input_line.trim_matches('\n');

    // Check if the pattern starts with the `^` anchor
    let starts_with_anchor = patterns.first() == Some(&Pattern::StartOfLine);

    let mut iter = input_line.chars();
    
    if starts_with_anchor {
        // If pattern starts with `^`, match the input from the beginning
        let input = &input_line[..];
        let mut input_iter = input.chars();
        for pattern in patterns.iter().skip(1) {
            match pattern {
                Pattern::Literal(l) => {
                    if !match_literal(&mut input_iter, *l) {
                        return false;
                    }
                }
                Pattern::Digit => {
                    if !match_digit(&mut input_iter) {
                        return false;
                    }
                }
                Pattern::Alphanumeric => {
                    if !match_alphanumeric(&mut input_iter) {
                        return false;
                    }
                }
                Pattern::Group(positive, group) => {
                    if match_group(&mut input_iter, group) != *positive {
                        return false;
                    }
                }
                _ => (),
            }
        }
        return input_iter.clone().count() == 0; // Ensure the entire input is consumed
    }

    // If pattern doesn't start with `^`, match anywhere in the input
    'input_iter: for i in 0..input_line.len() {
        let input = &input_line[i..];
        let mut input_iter = input.chars();
        for pattern in patterns.iter() {
            match pattern {
                Pattern::Literal(l) => {
                    if !match_literal(&mut input_iter, *l) {
                        continue 'input_iter;
                    }
                }
                Pattern::Digit => {
                    if !match_digit(&mut input_iter) {
                        continue 'input_iter;
                    }
                }
                Pattern::Alphanumeric => {
                    if !match_alphanumeric(&mut input_iter) {
                        continue 'input_iter;
                    }
                }
                Pattern::Group(positive, group) => {
                    if match_group(&mut input_iter, group) != *positive {
                        continue 'input_iter;
                    }
                }
                _ => (),
            }
        }
        return true;
    }

    false
}

fn build_group_pattern(iter: &mut Chars) -> (bool, String) {
    let mut group = String::new();
    let mut positive = true;
    if iter.clone().next().is_some_and(|c| c == '^') {
        positive = false;
        iter.next();
    }
    loop {
        let member = iter.next();
        if member.is_none() {
            panic!("Incomplete character group");
        }
        let member = member.unwrap();
        if member != ']' {
            group.push(member);
            continue;
        }
        break;
    }
    (positive, group)
}

fn build_patterns(pattern: &str) -> Vec<Pattern> {
    let mut iter = pattern.chars();
    let mut patterns = Vec::new();
    loop {
        let current = iter.next();
        if current.is_none() {
            break;
        }
        match current.unwrap() {
            '\\' => {
                let special = iter.next();
                if special.is_none() {
                    panic!("Incomplete special character")
                }
                match special.unwrap() {
                    'd' => patterns.push(Pattern::Digit),
                    'w' => patterns.push(Pattern::Alphanumeric),
                    '\\' => patterns.push(Pattern::Literal('\\')),
                    _ => panic!("Invalid special character"),
                }
            }
            '[' => {
                let (positive, group) = build_group_pattern(&mut iter);
                patterns.push(Pattern::Group(positive, group));
            }
            '^' => patterns.push(Pattern::StartOfLine),
            l => patterns.push(Pattern::Literal(l)),
        }
    }
    patterns
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }
    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

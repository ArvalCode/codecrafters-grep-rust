use std::env;
use std::io;
use std::process;
use std::str::Chars;

#[derive(Debug)]
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

    let mut iter = input_line.chars();
    let mut start_of_line = false;

    'input_iter: for i in 0..input_line.len() {
        if patterns.first() == Some(&Pattern::StartOfLine) {
            start_of_line = true;
        }

        if start_of_line && i == 0 {
            // Check if the input starts with the pattern
            let input = &input_line[i..];
            let mut iter = input.chars();
            for pattern in patterns.iter().skip(1) {
                match pattern {
                    Pattern::Literal(l) => {
                        if !match_literal(&mut iter, *l) {
                            continue 'input_iter;
                        }
                    }
                    Pattern::Digit => {
                        if !match_digit(&mut iter) {
                            continue 'input_iter;
                        }
                    }
                    Pattern::Alphanumeric => {
                        if !match_alphanumeric(&mut iter) {
                            continue 'input_iter;
                        }
                    }
                    Pattern::Group(positive, group) => {
                        if match_group(&mut iter, group) != *positive {
                            continue 'input_iter;
                        }
                    }
                    _ => (),
                }
            }
            return true;
        }
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

#!/usr/bin/env rust-script
use std::fs::{read_to_string, File};
use std::io::{prelude::*, BufReader, Result};

fn read_ident(s: &str, curr: &mut usize, peek: &mut usize, char: &mut char) -> String {
    let pos = *curr;
    let mut ch = *char;
    while ch.is_alphabetic() || ch == '\'' {
        read_char(s, curr, peek, char);
        ch = *char;
    }
    let diff = *curr - pos;
    String::from(&s[pos..pos + diff])
}

fn read_number(s: &str, curr: &mut usize, peek: &mut usize, char: &mut char) -> i32 {
    let pos = *curr;
    let mut ch: char = s.as_bytes()[*curr].into();
    while ch.is_numeric() {
        read_char(s, curr, peek, char);
        ch = s.as_bytes()[*curr].into();
    }
    let diff = *curr - pos;
    let string = &s[pos..pos + diff];
    return match string.parse::<i32>() {
        Ok(n) => n,
        _ => 0,
    };
}

fn read_char(s: &str, curr: &mut usize, peek: &mut usize, ch: &mut char) {
    let p = *peek;
    *curr = p;
    *peek = p + 1;
    if p > s.len() || *curr >= s.len() {
        *ch = '\0';
    } else {
        *ch = s.as_bytes()[*curr].into();
    }
}

fn peek_char(s: &str, peek: usize) -> char {
    if peek > s.len() {
        return '\0';
    }
    return s.as_bytes()[peek].into();
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut skip = false;

    let mut curr: usize = 0;
    let mut peek: usize = 1;
    let mut char: char = ' ';

    while peek < input.len() {
        while char.is_whitespace() {
            read_char(input.as_str(), &mut curr, &mut peek, &mut char);
        }
        match char {
            '\0' => break,
            'A'..='Z' | 'a'..='z' => {
                let ident = read_ident(input.as_str(), &mut curr, &mut peek, &mut char);
                match ident.as_str() {
                    "mul" => {
                        if skip {
                            continue;
                        }
                        if char != '(' {
                            continue;
                        }
                        read_char(input.as_str(), &mut curr, &mut peek, &mut char);
                        let n1 = read_number(input.as_str(), &mut curr, &mut peek, &mut char);
                        if char != ',' {
                            continue;
                        }
                        read_char(input.as_str(), &mut curr, &mut peek, &mut char);
                        let n2 = read_number(input.as_str(), &mut curr, &mut peek, &mut char);
                        if char != ')' {
                            continue;
                        }
                        read_char(input.as_str(), &mut curr, &mut peek, &mut char);
                        sum += n1 * n2;
                    }
                    "do" => {
                        if char != '(' {
                            continue;
                        }
                        read_char(input.as_str(), &mut curr, &mut peek, &mut char);
                        if char != ')' {
                            continue;
                        }
                        read_char(input.as_str(), &mut curr, &mut peek, &mut char);
                        skip = false;
                    }
                    "don't" => {
                        if char != '(' {
                            continue;
                        }
                        read_char(input.as_str(), &mut curr, &mut peek, &mut char);
                        if char != ')' {
                            continue;
                        }
                        read_char(input.as_str(), &mut curr, &mut peek, &mut char);
                        skip = true;
                    }
                    _ => {}
                }
                continue;
            }
            _ => {}
        }
        read_char(input.as_str(), &mut curr, &mut peek, &mut char);
    }
    println!("{}", sum);

    Ok(())
}

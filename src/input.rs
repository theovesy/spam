// Managing inputs
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn input_from_file(file_name: &String) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>>;

    // Opening the input file
    let path = Path::new(file_name);

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open input file: {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read input: {}", why),
        Ok(_) => input = parse_input(&s),
    }

    input
}

fn parse_input(s: &String) -> Vec<Vec<char>> {
    println!("Input file:\n{}", s);
    let s = s.as_str();
    let lines = s.lines();
    // Getting the longuest line's length
    // and the taget image size
    let mut max_len = 0;
    let mut line_number = 0;
    for line in lines {
        line_number += 1;
        let mut line_len = 0;
        for _ch in line.chars() {
            line_len += 1;
        }
        if line_len >= max_len {
            max_len = line_len;
        }
    }
    let imgx = max_len;
    let imgy = line_number;

    let mut input = vec![vec!['.'; imgx];imgy];
    let mut x = 0;
    let mut y = 0;

    for line in s.lines() {
        for ch in line.chars() {
            input[x][y] = ch;
            y += 1;
        }
        x += 1;
        y = 0;
     }

    input
}


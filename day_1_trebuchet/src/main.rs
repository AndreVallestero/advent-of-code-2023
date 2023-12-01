// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};

const NUMBER_WORDS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input")?);
    let mut total = 0;


    for line in reader.lines() {
        let line = line?;
        let mut start = None;
        let mut end = None;


        /* // Part 1
        for character in line.chars() {
            if character.is_numeric() {
                if start.is_none() {
                    start = Some(character);
                }
                end = Some(character);
            }
        }
        total += format!("{}{}", start.unwrap(), end.unwrap()).parse::<i32>().unwrap();
        */

        // Part 2
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if start.is_none() {
                    start = Some(char_to_int(c));
                }
                end = Some(char_to_int(c));
            } else {
                for (number, number_word) in NUMBER_WORDS.iter().enumerate() {
                    if i >= number_word.len() - 1 {
                        if **number_word == line[i+1-number_word.len()..i+1]{
                            if start.is_none() {
                                start = Some(number as i32);
                            }
                            end = Some(number as i32);
                        }
                    }
                }
            }
        }

        total += start.unwrap() * 10 + end.unwrap();
    }

    Ok(println!("Total {}", total))
}

fn char_to_int(character: char) -> i32 {
    character as i32 - '0' as i32
}
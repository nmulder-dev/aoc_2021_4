use std::error::Error;
use std::fs;
use std::env;
use std::str::FromStr;
use std::str;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip first arg
        args.next();

        // get filename from args or return error
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing input filename"),
        };

        Ok(Config {
            filename: filename,
        })
    }
}

pub struct Number {
    number: u32,
    marked: bool,
}

pub struct BingoCard<'a> {
    numbers: [[&'a Number; 5];5]
}

impl<'a> BingoCard<'a> {
    pub fn populate_from_str(card_numbers: [[&str; 5];5]) -> BingoCard {
        // Initialize struct
        let mut numbers: [[&Number; 5];5];        
        for (i, row) in card_numbers.into_iter().enumerate() {
            for (j,column) in row.into_iter().enumerate() {
                let num: u32 = FromStr::from_str(*column).unwrap();
                numbers[i][j] = &Number {
                    number: num,
                    marked: false
                };
            }
        }
        BingoCard { numbers: numbers }        
    }

    pub fn new() -> BingoCard {

    }

    fn has_winner(&self){

    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines: Vec<&str> = get_lines_vec(&contents);

    // YOUR CODE HERE!
    let call_numbers = get_call_numbers(&lines);
     
    let mut lines_iter = lines.into_iter();
    lines_iter.next();  // skip call number line

    let mut card_numbers: Vec<&Vec<&str>> = Vec::new();
    let mut row: Vec<&str> = Vec::new();

    let mut cards: Vec<BingoCard> = Vec::new();
    for (i, line) in lines_iter.enumerate() {
        
    }

    Ok(())
}

fn get_call_numbers(lines: &Vec<&str>) -> Vec<u32>
{
    let mut numbers: Vec<u32> = Vec::new();
    let line = lines[0].split(',');
    for number in line {
        numbers.push(FromStr::from_str(number).unwrap());
    }
    numbers
}

fn get_lines_vec(content: &str) -> Vec<&str> {
    // collect lines of content and return it
    content.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_length_greater_than_one() {
        let content = "\
Dolore eiusmod irure laboris consequat eiusmod irure ad occaecat aute qui veniam mollit officia officia.
Non irure nisi ut non cillum pariatur pariatur ex officia consectetur tempor adipisicing veniam. Dolore
irure deserunt aliquip minim incididunt ipsum eu mollit eiusmod Lorem qui dolor. Ipsum sint nostrud
anim labore ex adipisicing excepteur velit fugiat pariatur fugiat ex incididunt.";
        
        let test_vec = get_lines_vec(content);

        assert!(test_vec.len() > 1);
    }

    #[test]
    fn vector_length_equal_to_zero() {
        let content = "";

        let test_vec = get_lines_vec(content);

        assert_eq!(test_vec.len(), 0);
    }
}
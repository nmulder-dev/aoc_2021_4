use std::error::Error;
use std::fs;
use std::env;
use std::str::FromStr;
use std::str;
use std::io::{self, Write};

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

pub struct BingoCard {
    numbers: Vec<Vec<Number>>
}

impl BingoCard {
    pub fn new() -> BingoCard {
        // Return (0, false) initialized 5 x 5 vector of vectors
        BingoCard { 
            numbers: vec![vec![Number { number: 0, marked: false }; 5];5]
         }
    }

    fn is_empty(&self) -> bool {
        let mut empty = true;

        for number in self.numbers.iter() {
            for value in number {
                if value.number != 0 {
                    empty = false;
                }
            }
        }
        empty
    }

    fn has_winner(&self) {
    }
}

impl Copy for Number {}

impl Clone for Number {
    fn clone(&self) -> Self {
        *self
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines: Vec<String> = get_lines_vec(&contents);

    // YOUR CODE HERE!
    let mut cards: Vec<BingoCard> = Vec::new();
    let mut card = BingoCard::new();
    let mut test = "";
    let mut counter = 0;
    let mut temp = Number {number: 0, marked: false };

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            continue;
        } else if line == "" && !card.is_empty() {
            cards.push(card);
            card = BingoCard::new();
            counter = 0;
            println!();
        } else {
            if line == "" {
                // Reset counter
                counter = 0;
                card = BingoCard::new();
                continue;
            } 
            // Loop through numbers in current line
            for (j, num) in read_number_line(line).iter().enumerate() {
                let n = u32::from_str(num);
                // Check parse is Ok
                let num = match n {
                    Ok(num) => num,
                    Err(error) => panic!("An error occured: {:?}", error),
                };
                card.numbers[counter][j] = Number {
                    number: num,
                    marked: false,
                };
            }
            counter += 1;    
        }
    }    

    Ok(())
}

fn read_number_line (line: &str) -> Vec<String> {
    let mut numbers: Vec<String> = Vec::new();
    let mut number_vec = Vec::new();

    for (ch) in line.chars() {
        if ch != ' ' && ch != '\n'{
            print!("{}",ch);
            io::stdout().flush().unwrap();
            number_vec.push(ch);
        } else {
            print!(" ");
            io::stdout().flush().unwrap();
            if !number_vec.is_empty() {
                numbers.push(number_vec.iter().cloned().collect::<String>());
                number_vec = Vec::new();
            }
        }
    }
    // Push last buffered number
    numbers.push(number_vec.iter().cloned().collect::<String>());

    println!();
    numbers
}

fn get_call_numbers(lines: &Vec<&str>) -> Vec<u32>
{
    let mut bingo_card = BingoCard::new();

    let mut numbers: Vec<u32> = Vec::new();
    let line = lines[0].split(',');
    for number in line {
        numbers.push(FromStr::from_str(number).unwrap());
    }
    numbers
}

fn get_lines_vec(content: &str) -> Vec<String> {
    // collect lines of content and return it
    content.lines()
            .map(|s| s.to_string())
            .collect()
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
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
    numbers: Vec<Vec<&'a Number>>
}

impl<'a> BingoCard<'a> {
    pub fn new() -> BingoCard<'a> {
        let num_vec = vec![&Number { number: 0, marked: false }; 5];
        BingoCard { 
            numbers: vec![num_vec;5]
         }
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

    let mut cards: Vec<BingoCard> = Vec::new();
    let mut card: &BingoCard;
    let mut count = 0;
    for (i, line) in lines_iter.enumerate() {
        if line == "" {
            // Next line begins new card
            cards.push(BingoCard::new());
            card = cards.last().unwrap();

        } else {
            let numbers = line.split(",");
            for (j, number) in numbers.enumerate() {
                card.numbers[i][count] = &Number {
                    number: FromStr::from_str(number).unwrap();
                    mark
                }
            }
        }
    }

    Ok(())
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
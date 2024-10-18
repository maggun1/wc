use std::io::{BufRead, BufReader, Error, Read, Result};
use std::fs::File;
use std::process::exit;

fn count_characters(file_path: &str) -> Result<usize>
{
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string.chars().count())
}

fn count_lines(file_path: &str) -> Result<usize>
{
    let file = File::open(file_path)?;
    let bufreader = BufReader::new(file);
    Ok(bufreader.lines().count())
}

fn count_words(file_path: &str) -> Result<usize>
{
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string.split_whitespace().count())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (option, file_path) =
        if args.len() == 2 {
            ("-w", args[1].as_str())
        }
        else if args.len() == 3 {
            (args[1].as_str(), args[2].as_str())
        }
        else {
            println!("Invalid number of arguments!");
            println!("Usage: wc  [ -w | -c | -l ] <file-path>");
            exit(1);
        };

    let result = match option {
        "-w" => {count_words(&file_path)},
        "-c" => {count_characters(&file_path)},
        "-l" => {count_lines(&file_path)},
        _ => {
            println!("Invalid option!");
            println!("Usage: wc  [ -w | -c | -l ] <file-path>");
            exit(1);
        }
    };

    println!("{}", result.unwrap());
}

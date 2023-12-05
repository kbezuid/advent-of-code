use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

fn main() {
    let regex = Regex::new(r"[A-Za-z]").unwrap();
    let mut total = 0;
    let file_result = read_lines("input.txt");

    match file_result {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(cal) => {
                        let digits = regex.replace_all(&cal, "");
                        let first = digits.chars().next().unwrap();
                        let last = digits.chars().rev().next().unwrap();
                        let value = format!("{}{}", first, last);
                        let v = value.parse::<i32>().unwrap();
                        total += v;
                        println!("{}", value);
                    }
                    Err(err) => {
                        println!("Line Error: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    println!("Total: {}", total)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

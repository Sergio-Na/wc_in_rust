use regex::Regex;
use std::fs;
use std::io::{self, Read};

fn main() {
    let re = Regex::new(r"(?u)\b\w+\b").unwrap();
    let flag = std::env::args().nth(1);
    let filename_maybe = std::env::args().nth(2);
    let contents = match filename_maybe {
        Some(ref filename) => {
            fs::read_to_string(filename).expect("Should have been able to read the file")
        }
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read from stdin");
            input
        }
    };

    match flag {
        Some(pattern) => match pattern.as_str() {
            "-c" => print!("  {}", contents.len()),
            "-l" => print!("  {}", contents.matches('\n').count()),
            "-w" => print!("  {}", re.find_iter(&contents).count()),
            "-m" => print!("  {}", contents.chars().count()),
            _ => println!("blahh blahhh"),
        },
        None => {
            print!(
                "{} {} {}",
                contents.len(),
                contents.matches('\n').count(),
                contents.split(" ").count()
            )
        }
    }
    match filename_maybe {
        Some(ref filename) => println!(" {}", filename),
        _ => {}
    }
}

use std::fs::read_to_string;
use std::env::args;

fn main() {
    let filename = args().nth(1).expect("Input file not specified");

    println!("filename: {filename}");

    let file = read_to_string(&filename);

    let input = match file {
        Ok(input) => input,
        Err(error) => panic!("Error: {}", error),
    };



    let output = input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");

            let mut iter =
                line.chars().filter_map(|char| {
                    char.to_digit(10)
                });
            let first = iter.next().expect("should be number");
            let last = iter.last();
            
            match last {
                Some(num) => format!("{first}{num}").parse::<u32>(),
                None => format!("{first}{first}").parse::<u32>(),
            }.expect("should be valid number")
        })
        .sum::<u32>();

    println!("{output:?}");

}

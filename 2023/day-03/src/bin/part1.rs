use std::env::args;
use std::fs::read_to_string;


#[derive(Debug, PartialEq)]
enum Symbol {
    Gear(char),
    Digit(u32),
    Dot(),
}

#[derive(Debug)]
struct Char {
    char_type: Symbol,
}

fn main() {
    let filename = args().nth(1).expect("File name not specified");

    let input = read_to_string(filename).expect("Invalid input");

    println!("{input}");

    let arr: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for i in &arr {
        println!("{i:?}")
    }

    let pluh = &arr[6][4];
    let pluht;
    if pluh.is_numeric() {
        pluht = Char { char_type: Symbol::Digit(pluh.to_digit(10).unwrap()) };
    }
    else {
        pluht = Char { char_type: Symbol::Dot() }
    }

    if pluht.char_type != Symbol::Dot(){
    println!("{pluht:?}");}
    
    match pluht.char_type {
        Symbol::Gear(x) => println!("{x}"),
        Symbol::Digit(x) => println!("{x}"),
        Symbol::Dot() => println!("brah"),
    }


    println!("pluh: {pluh}\nploah: {ploa}", ploa=arr[6-1][4+1]);
}
// hashmap of digits with key as a struct Char with value of coords?
use std::fs::read_to_string;
use std::env::args;


#[derive(Debug)]
struct Game {
    id: u32,
    blues: u32,
    reds: u32,
    greens: u32,
}

impl Game {
    fn is_valid_game(game: &Game) -> bool {
        return game.blues <= MAX_BLUE && game.greens <= MAX_GREEN && game.reds <= MAX_RED
    }
    
    fn power_of_cubes(game: &Game) -> u32 {
        return game.blues * game.reds * game.greens
    }
}

const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;
const MAX_RED: u32 = 12;

fn main() {
    let filename = args().nth(1).expect("FIle name not specified");

    let contents = read_to_string(&filename).expect("Something went wrong reading the file");    

    let games: Vec<Game> = contents
    .lines()
    .map(|line| {
        
        let mut new_line = line.replace("Game ", "");
        
        let id = new_line.split(":").nth(0).unwrap().parse::<u32>().expect("id should be a number");        
        
        new_line = new_line.replace(new_line.split(" ").nth(0).unwrap(), "").strip_prefix(" ").expect("invalid input").to_string();

        let mut blues = 0;
        let mut greens = 0;
        let mut reds = 0;

        let _ = new_line.split("; ").for_each(|game| {
            game.split(", ").for_each(|color| {
                let (amount, color_name) = (color.split(" ").nth(0).unwrap(), color.split(" ").nth(1).unwrap());
                let _ = match color_name {
                    "blue" => { if &amount.parse::<u32>().unwrap() > &blues { blues = amount.parse().unwrap() }}
                    "red" => { if &amount.parse::<u32>().unwrap() > &reds { reds = amount.parse().unwrap() }}
                    "green" => { if &amount.parse::<u32>().unwrap() > &greens { greens = amount.parse().unwrap() }}
                    _ => panic!("wtf")

                };
            })
        });
        Game {id, blues, reds, greens}
    }).collect();

    let mut sum: u32 = 0;

    games.iter().for_each(|game| {
        sum += Game::power_of_cubes(game)
    });
    println!("Sum of powers: {sum}");
} 
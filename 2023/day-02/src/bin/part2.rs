use std::fs::read_to_string;
use std::env::args;


#[derive(Debug)]
struct Game {
    blues: u32,
    reds: u32,
    greens: u32,
}

impl Game {   
    fn power_of_cubes(game: &Game) -> u32 {
        return game.blues * game.reds * game.greens
    }
}

fn main() {
    let filename = args().nth(1).expect("FIle name not specified");

    let contents = read_to_string(&filename).expect("Something went wrong reading the file");    

    let games: Vec<Game> = contents
    .lines()
    .map(|line| {
        
        let mut new_line = line.replace("Game ", "");
                
        new_line = new_line.replace(new_line.split(" ").nth(0).unwrap(), "").trim().to_string();

        let mut blues = 0;
        let mut greens = 0;
        let mut reds = 0;

        let _ = new_line.split("; ").for_each(|game| {
            game.split(", ").for_each(|color| {
                let (amount, color_name) = (color.split(" ").nth(0).unwrap(), color.split(" ").nth(1).unwrap());
                match color_name {
                    "blue" => { blues = blues.max(amount.parse().unwrap()) }
                    "red" => { reds = reds.max(amount.parse().unwrap()) }
                    "green" => { greens = greens.max(amount.parse().unwrap()) }
                    _ => panic!("wtf")
                };
            })
        });
        Game {blues, reds, greens}
    }).collect();

    let mut sum: u32 = 0;

    games.iter().for_each(|game| {
        sum += Game::power_of_cubes(game)
    });
    println!("Sum of powers: {sum}");
} 
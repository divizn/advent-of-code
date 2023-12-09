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
        Game {id, blues, reds, greens}
    }).collect();

    let mut sum: u32 = 0;

    games.iter().for_each(|game| {
        if Game::is_valid_game(game) {
            sum += game.id;
        }
    });

    println!("Sum of ids: {sum}");
} 
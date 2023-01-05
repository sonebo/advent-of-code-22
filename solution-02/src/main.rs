use std::{io::{BufReader, self, BufRead}, fs::File};

struct Round {
    opp_shape: Shape,
    outcome: Outcome,
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

fn shape_score(shape: Shape) -> i32 {
    return match shape {
        Shape::Rock => 1, 
        Shape::Paper => 2, 
        Shape::Scissors => 3,
    }
}

fn round_score(opp_shape: Shape, outcome: Outcome) -> i32 {
    let round = Round {opp_shape, outcome};    
    return match round {
        Round {opp_shape: Shape::Rock, outcome: Outcome::Lose} => shape_score(Shape::Scissors) + 0,
        Round {opp_shape: Shape::Rock, outcome: Outcome::Draw} => shape_score(Shape::Rock) + 3,
        Round {opp_shape: Shape::Rock, outcome: Outcome::Win} => shape_score(Shape::Paper) + 6,
        Round {opp_shape: Shape::Paper, outcome: Outcome::Lose} => shape_score(Shape::Rock) + 0,
        Round {opp_shape: Shape::Paper, outcome: Outcome::Draw} => shape_score(Shape::Paper) + 3,
        Round {opp_shape: Shape::Paper, outcome: Outcome::Win} => shape_score(Shape::Scissors) + 6,
        Round {opp_shape: Shape::Scissors, outcome: Outcome::Lose} => shape_score(Shape::Paper) + 0,
        Round {opp_shape: Shape::Scissors, outcome: Outcome::Draw} => shape_score(Shape::Scissors) + 3,
        Round {opp_shape: Shape::Scissors, outcome: Outcome::Win} => shape_score(Shape::Rock) + 6,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("test-input.txt")?;
    let reader = BufReader::new(file);
    let mut total_score: i32 = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let shape_vec: Vec<char> = l.chars().collect();
                let outcome = match shape_vec[2] {
                    'X' => Outcome::Lose,
                    'Y' => Outcome::Draw,
                    'Z' => Outcome::Win,
                    _ => panic!("An invalid outcome was supplied"),
                };
                let opp_shape = match shape_vec[0] {
                    'A' => Shape::Rock,
                    'B' => Shape::Paper,
                    'C' => Shape::Scissors,
                    _ => panic!("An invalid shape was supplied"),
                };
                total_score += round_score(opp_shape, outcome);
            },
             Err(_e) => println!("error"),   
        }
    }
    println!("{:?}", total_score);
    Ok(())
}

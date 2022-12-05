use std::{io::{BufReader, self, BufRead}, fs::File};

struct Round {
    shape_1: char,
    shape_2: char,
}

fn shape_score(shape_2: char) -> i32 {
    return match shape_2 {
        'X' => 1, 
        'Y' => 2, 
        'Z' => 3,
        _ => {
            println!("Unexpected value given for shape_2");
            0
        }
    }
}

fn round_score(shape_1: char, shape_2:char) -> i32 {
    let round = Round {shape_1, shape_2};    
    return match round {
        Round {shape_1: 'A', shape_2: 'X'} => shape_score(shape_2) + 3,
        Round {shape_1: 'A', shape_2: 'Y'} => shape_score(shape_2) + 6,
        Round {shape_1: 'A', shape_2: 'Z'} => shape_score(shape_2) + 0,
        Round {shape_1: 'B', shape_2: 'X'} => shape_score(shape_2) + 0,
        Round {shape_1: 'B', shape_2: 'Y'} => shape_score(shape_2) + 3,
        Round {shape_1: 'B', shape_2: 'Z'} => shape_score(shape_2) + 6,
        Round {shape_1: 'C', shape_2: 'X'} => shape_score(shape_2) + 6,
        Round {shape_1: 'C', shape_2: 'Y'} => shape_score(shape_2) + 0,
        Round {shape_1: 'C', shape_2: 'Z'} => shape_score(shape_2) + 3,
        _ => {
            println!("Unexpected value in the pair of shapes given to round_score");
            0
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total_score: i32 = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let shape_vec: Vec<char> = l.chars().collect();
                total_score += round_score(shape_vec[0], shape_vec[2]);
            },
             Err(_e) => println!("error"),   
        }
    }
    println!("{:?}", total_score);
    Ok(())
}

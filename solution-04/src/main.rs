use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn vec_to_tuple(vec: Vec<&str>) -> ((i32, i32), (i32, i32)) {
    let t1_vec: Vec<&str> = vec[0].split("-").collect();
    let t2_vec: Vec<&str> = vec[1].split("-").collect();

    return (
        (
            t1_vec[0].parse::<i32>().unwrap(),
            t1_vec[1].parse::<i32>().unwrap(),
        ),
        (
            t2_vec[0].parse::<i32>().unwrap(),
            t2_vec[1].parse::<i32>().unwrap(),
        ),
    );
}

fn fully_contains(tuple: ((i32, i32), (i32, i32))) -> bool {
    let (i1, i2) = tuple;
    let (i1_start, i1_end) = i1;
    let (i2_start, i2_end) = i2;
    
    if i1_start <= i2_start && i1_end >= i2_end {
        return true
    }

    if i2_start <= i1_start && i2_end >= i1_end {
        return true
    }

    return false;
}

fn overlaps(tuple: ((i32, i32), (i32, i32))) -> bool {
    let (i1, i2) = tuple;
    let (i1_start, i1_end) = i1;
    let (i2_start, i2_end) = i2;
    
    if i1_end < i2_start {
        return false
    }

    if i2_end < i1_start {
        return false
    }

    return true;
}


fn main() -> io::Result<()> {
    // create two tuples (a,b) and (x,y) for each line of input
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut all_tuples: Vec<((i32, i32), (i32, i32))> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let vec: Vec<&str> = l.split(",").collect();
                all_tuples.push(vec_to_tuple(vec));
            }
            Err(_) => todo!(),
        };
    }
    let fully_contained_pairs:Vec<&((i32, i32), (i32, i32))> = all_tuples
        .iter()
        .filter(|tuple: &&((i32, i32), (i32, i32))| overlaps(**tuple))
        .collect();
    println!("{:?}", fully_contained_pairs.len());
    Ok(())
}

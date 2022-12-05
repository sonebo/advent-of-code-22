
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut elf_vec:Vec<i32>= Vec::new();
    
    let mut max = 0;
    let mut sum: i32 = 0;
    for line in reader.lines() {
        let num: i32;
         match line {
            Ok(l) => {
                let l = l.trim().to_string();
                if l.is_empty() {
                    elf_vec.push(sum);
                    sum = 0
                } else {
                    num = l.parse().unwrap();
                    sum += num;
                }
                
            },
            Err(_e) => println!("error"),
         }
         if sum > max {
            max = sum
         }
         
    }
    // sort in descending order
    elf_vec.sort_by(|a, b| b.cmp(a));

    let sum_top_three = elf_vec[0] + elf_vec[1] + elf_vec[2];
    println!("{}", max.to_string());
    println!("{:?}", sum_top_three);
    Ok(())
} 


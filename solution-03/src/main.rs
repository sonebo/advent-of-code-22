use core::panic;
use std::{io::{BufReader, self, BufRead}, fs::File, collections::{BTreeMap}};
use std::collections::HashSet;

fn vec_to_set(vec: Vec<char>) -> HashSet<char> {
    HashSet::from_iter(vec)
}

fn get_priority(c: char) -> u32 {
    if c as u32 >= 'A' as u32 && c as u32 <= 'Z' as u32 {
        return c as u32 - 'A' as u32 + 1;
    } else if c as u32 >= 'a' as u32 && c as u32 <= 'z' as u32 {
        return c as u32 - 'a' as u32 + 1;
    }
    panic!("Got invalid char")
}

fn main() -> io::Result<()> {

    let mut priority:BTreeMap<char,usize> = BTreeMap::new();
    let mut ascii_lowercase = ('a'..='z').collect::<Vec<char>>();
    let ascii_uppercase = ('A'..='Z').collect::<Vec<char>>();

    ascii_lowercase.extend(&ascii_uppercase);
    
    for(i, item) in ascii_lowercase.iter().enumerate(){
        priority.insert(*item, i+1);
    }

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut group_sum:usize = 0;
    let mut group_rucksacks: Vec<HashSet<char>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                let rucksack_items: Vec<char> = l.chars().collect();
                if (i + 1) % 3 == 0 {
                    group_rucksacks.push(vec_to_set(rucksack_items));
                    let group_item = group_rucksacks[0].intersection(&group_rucksacks[1]).find(|it| group_rucksacks[2].contains(it));
                    group_sum += priority.get(group_item.unwrap()).unwrap();
                    group_rucksacks.clear();
                } else {
                     group_rucksacks.push(vec_to_set(rucksack_items));
                }
                
            },
             Err(_e) => println!("error"),   
        }
    }
    println!("{:?}", group_sum);
    Ok(())
}


fn _part_one() -> io::Result<()> {

    let mut priority:BTreeMap<char,usize> = BTreeMap::new();
    let mut ascii_lowercase = ('a'..='z').collect::<Vec<char>>();
    let ascii_uppercase = ('A'..='Z').collect::<Vec<char>>();

    ascii_lowercase.extend(&ascii_uppercase);
    
    for(i, item) in ascii_lowercase.iter().enumerate(){
        priority.insert(*item, i+1);
    }

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let mut items: Vec<char> = l.chars().collect();
                let rucksack_size = items.len();

                let comp_2 = vec_to_set(items.split_off(rucksack_size / 2));
                let comp_1 = vec_to_set(items);

                let intersection = comp_1.intersection(&comp_2).collect::<Vec<&char>>();
                let common_item = intersection[0];
                
                sum += priority.get(common_item).unwrap()
            },
             Err(_e) => println!("error"),   
        }
    }
     println!("{:?}", sum);
    
    Ok(())
}

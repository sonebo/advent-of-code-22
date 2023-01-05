use std::{
    fs::File,
    io::{self, BufRead, BufReader}, collections::BTreeMap,
};
fn main() -> io::Result<()> {
    // create two tuples (a,b) and (x,y) for each line of input
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut stacks: BTreeMap<usize,Vec<char>> = BTreeMap::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let char_vec: Vec<char> = l.chars().collect();
                if char_vec.len() == 0 {
                    continue;
                }
                if  char_vec[1] == '1' {
                    let vals = stacks.values_mut().enumerate();
                    for val in vals {
                        val.1.reverse();
                    }
                    continue;
                }
                if char_vec[0] == 'm' {
                    let instruction: Vec<&str> = l.split(" ").collect();
                    let amount: usize = instruction[1].parse::<usize>().unwrap();
                    let source_stack: usize = instruction[3].parse::<usize>().unwrap() - 1;
                    let dest_stack: usize = instruction[5].parse::<usize>().unwrap() - 1;
                    let mut source = stacks.get(&source_stack).unwrap().clone();
                    let mut dest = stacks.get(&dest_stack).unwrap().clone();
                    
                    let mut intermediate: Vec<char> = Vec::new();
                    let mut j = 0;
                    while j < amount {
                        intermediate.push(source.pop().unwrap());
                        j += 1; 
                    }
                    intermediate.reverse();
                    dest = [dest, intermediate].concat();
                    stacks.insert(source_stack, source);
                    stacks.insert(dest_stack, dest);
                    continue;
                }
                let mut i = 0;
                while i < (char_vec.len() + 1) / 4 {
                    let pos = 1 + i * 4;
                    if char_vec[pos] == ' ' {
                        i+=1; 
                        continue;
                    }
                    if let None = stacks.get_mut(&i) {
                        stacks.insert(i, Vec::new());
                    }
                    stacks.get_mut(&i).map(|val| val.push(char_vec[pos]));
                    i+=1;   
                }
                            }
            Err(_) => todo!(),
        };
    }
    let values: String = stacks.values().map(|val| *val.last().unwrap()).collect();
    println!("{:?}", values);
    Ok(())
}

use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
struct InputFile {
    name: String,
    size: usize,
}

fn get_dir_size(
    i: usize,
    files_of: &BTreeMap<usize, Vec<InputFile>>,
    subdirs_of: &BTreeMap<usize, BTreeMap<String, usize>>,
) -> usize {
    let mut sum = 0;
    match files_of.get(&i) {
        None => {}
        Some(v) => {
            for file in v {
                sum += file.size;
            }
        } 
    }
    match subdirs_of.get(&i) {
        None => {}
        Some(v) => {
             for subdir in v {
                sum += get_dir_size(*subdir.1, files_of, subdirs_of);
            }
        }
    }
    return sum;
}

fn main() -> io::Result<()> {
    let file: File = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let root_dir: usize = 0;
    let mut current_dir: usize = root_dir;

    let mut parent_of: BTreeMap<usize, usize> = BTreeMap::new();
    let mut subdirs_of: BTreeMap<usize, BTreeMap<String, usize>> = BTreeMap::new();
    let mut files_of: BTreeMap<usize, Vec<InputFile>> = BTreeMap::new();

    subdirs_of.insert(root_dir, BTreeMap::new());

    let mut dir_counter: usize = 0;

    let mut get_next_dir = || -> usize {
        dir_counter += 1;
        dir_counter
    };

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let char_vec: Vec<char> = l.chars().collect();
                let words: Vec<&str> = l.split(" ").collect();
                if char_vec[0] == '$' {
                    if words[1] == "cd" {
                        let dir = words[2];
                        if dir == "/" {
                            current_dir = root_dir;
                        } else if dir == ".." {
                            current_dir = *parent_of.get(&current_dir).unwrap();
                        } else {
                            current_dir = *subdirs_of.get(&current_dir).unwrap().get(dir).unwrap();
                        }
                    }
                } else {
                    if words[0] == "dir" {
                        let new_dir = get_next_dir();
                        let m = subdirs_of
                            .get_mut(&current_dir);
                        match m {
                            None => {
                                let mut map = BTreeMap::new();
                                map.insert(String::from(words[1]), new_dir);
                                subdirs_of.insert(current_dir, map);
                            },
                            Some(x) => {
                                x.insert(String::from(words[1]), new_dir);
                            }
                        }
                        parent_of.insert(new_dir, current_dir);
                    } else {
                        let file_size: usize = words[0].parse::<usize>().unwrap();
                        let file_name = words[1];
                        let input_file: InputFile = InputFile {
                            name: String::from(file_name),
                            size: file_size,
                        };
                        match files_of.get_mut(&current_dir) {
                            None => {
                                files_of.insert(current_dir, vec![input_file]);
                            }
                            Some(vec) => {
                                vec.push(input_file);
                            }
                        }
                    }
                }
            }
            Err(_) => todo!(),
        }
    }

    let wanted_space = 30000000;
    let current_space = 70000000 - get_dir_size(root_dir, &files_of, &subdirs_of);
    let to_delete = wanted_space - current_space;

    let mut answer = get_dir_size(root_dir, &files_of, &subdirs_of);
    let mut i = 0;
    while i <= dir_counter {
        let dir_size = get_dir_size(i, &files_of, &subdirs_of);
        if dir_size >= to_delete && dir_size < answer {
            answer = dir_size;
        }
        i += 1;
    }
    println!("{:?}", subdirs_of);
    println!("{:?}", files_of);
    println!("{}", answer);
    Ok(())
}

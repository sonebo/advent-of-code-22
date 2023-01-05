use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut input_string = String::new();
    match file.read_to_string(&mut input_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let input_chars: Vec<char> = input_string.chars().collect::<Vec<_>>();
            let mut i = 0;
            while i < input_chars.len() {
                let mut set:HashSet<char> = HashSet::new();
                let mut j = 0;
                while j < 14 {
                    set.insert(input_chars[i+j]);
                    j += 1;
                }
                if set.len() == 14 {
                     println!("{:?}", i+j);
                     return;
                }
                i += 1;
            }
            

            
        },
    }


    // `file` goes out of scope, and the "hello.txt" file gets closed
}
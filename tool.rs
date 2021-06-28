 use std::env;
 use std::fs::*;
 use std::io::{BufRead, BufReader};
 use std::fs;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let new_File = &args[2];

    let file = File::open(filename).unwrap();
    let contents = BufReader::new(file);
    let mut output = String::default();

    for (index, line) in contents.lines().enumerate() {
       let line = line.unwrap(); 
       let actual = index + 1;
       
        if line.contains("unsafe"){
            output = output + &actual.to_string() + " : " + "Unsafe Code " + "\n";
        } 
    }

    fs::write(new_File , output).expect("Cannot find file");
}

use regex::Regex;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let re = Regex::new(r"mul\(([0-9]|[1-9][0-9]|[1-9][0-9][0-9]),([0-9]|[1-9][0-9]|[1-9][0-9][0-9])\)|(do\()|(don't)").unwrap();
    let single_list = fs::read_to_string("src/files/memory.txt").expect("Unable to read file");

    let result: Vec<_> = re.find_iter(single_list.as_str()).map(|m| m.as_str()).collect();
    //println!("{:?}", result);

    let mut sum = 0;
    let re = Regex::new(r"\d+").unwrap(); 
    // True do false dont
    let mut dos = true;
    for i in result {
        if i == "do(" {
            dos = true;
        }
        else if i == "don't" {
            dos = false;
        }
        let numbers: Vec<i32> = re
        .find_iter(i)
        .map(|mat| mat.as_str().parse().unwrap())
        .collect();
        if dos && !numbers.is_empty() {
            sum += numbers[0] * numbers[1];
        }
    
    }
    println!("{}", sum);

    println!("{:?}", start.elapsed());
}

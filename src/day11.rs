//rocks are my one and only friend
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    //let file = fs::read_to_string("src/files/stones.txt").expect("Unable to read file");
    let file = "125 17";
    let mut stones = file.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i32>>();
    println!("{:?}", start.elapsed());
    for b in 0..=1 {
        stones = rules(stones);
    }
    println!("{:?}", stones);
    println!("{:?}", start.elapsed());
}

fn rules(stones: Vec<i32>) -> Vec<i32> {
    let mut new_stones = stones;
    let mut i = 0;
    while i < new_stones.len() {
        if new_stones[i] == 0 {
            new_stones[i] = 1;
            i+=1;
            continue;
        }
        else if new_stones[i].to_string().len() % 2 == 0 {
            let stone = new_stones[i].to_string();
            let stone = stone.split_at((new_stones)[i].to_string().len()/2);
            new_stones[i] = stone.0.parse().unwrap();
            new_stones.insert(i+1, stone.1.parse().unwrap());
            i+=2;
            continue;
        }
        else {
            new_stones[i] = new_stones[i] * 2024;
            i+=1;
            continue;
        }
    }
    new_stones
}
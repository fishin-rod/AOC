//rocks are my one and only friend
// I ran part 2 for 2 hours and uhhh its being bad
use std::fs;
use std::time::Instant;
use cached::proc_macro::cached;

fn main() {
    let start = Instant::now();
    let file = fs::read_to_string("src/files/stones.txt").expect("Unable to read file");
    //let file = "125 17";
    let mut stones = file.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<usize>>();

    //for i in 0..75 {
        //stones = rules(stones);
        //println!("{:?}, i: {}", stones.len(), i);

    //}
    let stones = stones.iter().map(|n| blink(*n, 75)).sum::<usize>();
    //println!("{:?}", stones);
    println!("{:?}", stones);
    println!("{:?}", start.elapsed());
}
// use some cached code
#[cached]
fn blink(number: usize, remaining: usize) -> usize {
    if remaining == 0 {
        return 1;
    }

    let s = number.to_string();
    if number == 0 {
        blink(1, remaining - 1)
    } else if s.len() % 2 == 0 {
        // Split the digits of the number in half and run blink on both halves.
        let half = s.len() / 2;
        let left = s[..half].parse::<usize>().unwrap();
        let right = s[half..].parse::<usize>().unwrap();
        blink(left, remaining - 1) + blink(right, remaining - 1)
    } else {
        // Otherwise we just multiply by 2024.
        blink(number * 2024, remaining - 1)
    }
}

#[cached]
fn rules(stones: Vec<i128>) -> Vec<i128> {
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
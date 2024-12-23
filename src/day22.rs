// were so back
// please get that reference someone

use std::fs;
use std::time::Instant;

// how many times have I said "Oh I need a hashmap" and then proceeded to not use one
use std::collections::{HashMap, VecDeque};

fn main() {
    // This is more about reading comprehension then solving the problem
    // that was easy  ~30 mins of coding for part 1
    // I lied part 2 is kinda difficult coding wise and conceptually
    // I again lied IT WAS NOT EASY and I was following the wrong path
    let start = Instant::now();
    let file = fs::read_to_string("src/files/secret_numbers.txt").expect("Unable to read file");

    let file = file.lines().map(|x| x.parse().unwrap()).collect::<Vec<i128>>();
    //let file = vec![1, 2, 3, 2024];
    let a = puzzle_2(file.clone());
    println!("{:?}", a);
    let mut sum = 0;
    for i in file {
        sum += Db::p1(i, 0);
    }
    println!("{:?}", sum);
    println!("{:?}", start.elapsed());
}

struct Db {
    pats: Vec<Vec<i128>>,
    seqs: Vec<Vec<i128>>,
}

impl Db {
    fn new() -> Db {Db { pats: Vec::new(), seqs: Vec::new()}}
// maybe recursive wasnt the best idea but we are sticking to it!
// I did not stick to it
fn p1(mut i: i128, mut x: i128) -> i128 {
    while x < 2000 {
        let first = prune(mix(i, i * 64));
        let second = prune(mix(first, (first as f32 / 32.0).floor() as i128));
        // I hate 2024 time to move to 2025 because it doesnt look like 2048
        let third = prune(mix(second, second * 2048));
        
        // Update variables for the next iteration
        i = third;
        x += 1;
    }
    i
}
}
// copying code is letting people miss out on the crazyness I had originally developed :) 
 fn puzzle_2(input: Vec<i128>) -> String {

    let mut sequences_to_sum: HashMap<(i8, i8, i8, i8), (usize, u64)>  = HashMap::new();

    let mut window: VecDeque<i8> = VecDeque::new();
    let mut id: usize = 0;
    for line in input {
        let number = line as u64;
        let mut curr = number;
        let mut prev: Option<u64>;
        // WHY idk? But at this point im more then a little lost and just kinda copying sorry Ryan
        for _i in 0..2000 {
            prev = Some(curr);
            curr = gen_num(curr as i128);

            if let Some(prev) = prev {
                let change = (curr % 10) as i8 - (prev % 10) as i8;  // Thats way smarter then n.to_string().as_bytes().last().unwarp() as i128-48
                window.push_back(change);
                if window.len()  == 4 {

                    let price =  curr % 10;
                    let entry = sequences_to_sum.entry((window[0], window[1], window[2], window[3])).or_insert((id, price)); // // ATleast we were on a similar brainwave here
                    if (*entry).0 != id { // overwrite, update curr and sum // not me
                        *entry = (id,  price + (*entry).1);
                    }
                    window.pop_front();
                }
            }
        }
        id += 1;
        window.clear();
    }

    let mut best_sequence: Option<(i8, i8, i8, i8)> = None;
    let mut best_sequence_bananas: u64 = 0;
    for entry in sequences_to_sum {
        let max_bananas = entry.1.1;
        if max_bananas > best_sequence_bananas {
            best_sequence = Some(entry.0);
            best_sequence_bananas = max_bananas;
        }
    }

    dbg!(best_sequence_bananas);
    dbg!(best_sequence);
    best_sequence_bananas.to_string()
}

fn gen_num(i: i128) -> u64 {
    let first = prune(mix(i, i * 64));
    let second = prune(mix(first, (first as f32 / 32.0).floor() as i128));
    prune(mix(second, second * 2048)) as u64
}


fn mix(x: i128, i: i128) -> i128 {
    x ^ i
}

fn prune(x: i128) -> i128 {
    x % 16777216
}
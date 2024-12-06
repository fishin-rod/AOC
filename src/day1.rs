use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let (mut l_list, mut r_list) = lists();
    l_list.sort();
    r_list.sort();

    let mut sum = 0;
    for i in 0..l_list.len() {
        sum += (l_list[i] - r_list[i]).abs();
    }
    println!("{}", sum);

    let mut sim_score = 0;
    for i in l_list {
        sim_score += i * num_occurs(i, r_list.clone());
    }
    println!("{}", sim_score);

    println!("{:?}", start.elapsed());
}

fn num_occurs(num: i32, l: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in l {
        if i == num {
            count += 1;
        }
    }
    count
}

fn lists() -> (Vec<i32>, Vec<i32>) {
    let mut l_list = Vec::new();
    let mut r_list = Vec::new();
    let single_list = fs::read_to_string("src/files/locations.txt").expect("Unable to read file");
    let mut i = 0;
    for num in single_list.split_whitespace() {
        if i % 2 == 0 {
            l_list.push(num.parse().unwrap());
        }
        else {
            r_list.push(num.parse().unwrap());
        }
        i += 1;
    }
    (l_list, r_list)
}

use std::fs;
use std::time::Instant;
 
fn main() {
    let start = Instant::now();
    let file = fs::read_to_string("src/files/equations.txt").expect("Unable to read file");
    let mut sum = 0;
    for line in file.lines() {
        let total: Vec<&str> = line.split(':').collect();
        let left: i128 = total[0].parse().unwrap();
        let right: Vec<i32> = total[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        if test_nums(right, left) {
            sum += left;
        }
    }
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}

fn test_nums(nums: Vec<i32>, num: i128) -> bool {
    let total_combs = 3_i32.pow((nums.len()-1) as u32);
    for i in 0..total_combs {
        let mut temp = i; 
        let mut total: i128 = nums[0] as i128;
        for j in 0..(nums.len()-1) {
            let op = temp % 3;
            temp = temp / 3;
            match op {
                0 => total += nums[j+1] as i128,
                1 => total *= nums[j+1] as i128,
                2 => {
                    let con = format!("{}{}", total, nums[j+1]);
                    total = con.parse::<i128>().unwrap()
                }
                _ => panic!(),
            }
        }
        if total as i128 == num {
            return true;
        }
    }
    false
}
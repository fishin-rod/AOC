use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    let rules = create_rules();
    println!("{:?}", rules);
    let correct = check_order(&rules);
    println!("{:?}", correct.len());
    let sum = middles(correct);
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}

fn middles(vec: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for vec in vec {
        sum += vec[vec.len() / 2];
    }
    sum
}

fn check_order(rules: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut corrects = Vec::new();
    let file_content = fs::read_to_string("src/files/updates.txt").expect("Unable to read file");
    for line in file_content.lines() {
        println!("{}", line);
        let values = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut order = true;
        for i in 0..=values.len()-1 {
            println!("{:?}", rules.iter().position(|x| x == &values[i]));
            if let Some(pos) = rules.iter().position(|x| x == &values[i]) {
                if !i < pos {
                    order = false;
                }
            }
        }
        if order {
            println!("{:?}", values);
            corrects.push(values);
        }
    }
    corrects
}
// better idea create vector and push in the values right before the value is added
//after check if the number already comes before and leave or remove and move back
fn create_rules() -> Vec<i32> {
    let file_content = fs::read_to_string("src/files/rules.txt").expect("Unable to read file");
    let mut vec = Vec::new();
    for line in file_content.lines() {
        println!("{}", line);
        let values =line.split_at(2);
        let v0 = values.0.parse::<i32>().unwrap();
        let v1 = values.1.replace("|", "").parse::<i32>().unwrap();
        if vec.contains(&v1) && vec.contains(&v0) {
            // safe to unwrap because we know that the vector contains both values
            let index0 = vec.iter().position(|&x| x == v0).unwrap();
            let index1 = vec.iter().position(|&x| x == v1).unwrap();
            if index0 < index1 {
                continue;
            }
            else {
                vec.remove(index1);
                vec.insert(index1, v0);
            }
        }
        else if vec.contains(&v0) {
            let index = vec.iter().position(|&x| x == v0).unwrap();
            vec.insert(index+1, v1);
        }
        else if vec.contains(&v1) {
            let index = vec.iter().position(|&x| x == v1).unwrap();
            vec.insert(index-1, v0);
        }
        else {
            vec.push(v0);
            vec.push(v1);
        }
    }
    vec
}


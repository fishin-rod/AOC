//IM SO HAPPY I CAN DO IT
// carmer rule
use std::fs;
use std::time::Instant;
// guess ill have to do this to :cry:
use regex::Regex;

fn main() {
    let start = Instant::now();

    let file = fs::read_to_string("src/files/soeqs.txt").expect("Unable to read file");
    let file: Vec<&str> = file.lines().collect();
    //let file = ["Button A: X+94, Y+34", "Button B: X+22, Y+67", "Prize: X=8400, Y=5400", "", "Button A: X+26, Y+66", "Button B: X+67, Y+21", "Prize: X=12748, Y=12176", "", "Button A: X+17, Y+86", "Button B: X+84, Y+37", "Prize: X=7870, Y=6450", "", "Button A: X+69, Y+23", "Button B: X+27, Y+71", "Prize: X=18641, Y=10279"];
    // create vec with form
    //[r1_x, r1_y] [r1_s]
    //[r2_x, r2_y] [r2_s]
    let mut tot = 0;
    for i in {0..file.len()}.step_by(4) {
        //14932 ish to 32966
        let mut sols = vec![get_nums(file[i])]; 
        sols.push(get_nums(file[i+1]));
        let mut sol = get_nums(file[i+2]); // solutions
        sol[0] = sol[0] + 10000000000000;
        sol[1] = sol[1] + 10000000000000;
        let d = det(sols.clone()); 
        let mx = vec![vec![sol[0], sols[1][0]], vec![sol[1], sols[1][1]]];
        let dx = det(mx); // the borrow checker is throwing a fit
        let my = vec![vec![sols[0][0], sol[0]], vec![sols[0][1], sol[1]]];
        let dy = det(my);
        let x = dx as f64 / d as f64;
        let y = dy as f64 / d as f64;
        //if x < 0.0 || y < 0.0 {continue;}
        //if x > 100.0 || y > 100.0 {continue;}
        if x % 1.0 != 0.0 || y % 1.0 != 0.0 {continue;}
        println!("{:?}", x);
        println!("{:?}", y);
        tot += 3*x as i128;
        tot += y as i128;
        println!("{}", tot);
    }
    println!("{}", tot);
    println!("{:?}", start.elapsed());
}

fn det(mx: Vec<Vec<i128>>) -> i128 {
    mx[0][0] * mx[1][1] - mx[0][1] * mx[1][0]
}

fn get_nums(line: &str) -> Vec<i128> {
    let re = Regex::new(r"(\d+)").unwrap();
    re.captures_iter(line).map(|x| x[1].parse().unwrap()).collect() 
}
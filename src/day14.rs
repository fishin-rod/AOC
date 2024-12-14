use std::time::Instant;
use std::fs;
use regex::Regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

// Smart 
#[derive(Debug, Clone)]
struct Guard {
    x: i32,
    y: i32,
    v: (i32, i32)
}

// Did take some help by a smarter person to get the basics of the implimentation 
impl Guard {
    fn wrap(pos: i32, lim: i32) -> i32 {
        // this is a formula to beat all formulas
        ((pos % lim) + lim) % lim
    }
    fn sec(&mut self, ticks: i32) {
        self.x = Guard::wrap(self.x + self.v.0*ticks,  WIDTH);
        self.y = Guard::wrap(self.y + self.v.1*ticks,  HEIGHT); 
    }
    fn sx(&mut self) {
        self.x = Guard::wrap(self.x + self.v.0, WIDTH);
    }
    fn sy(&mut self) {
        self.y = Guard::wrap(self.y + self.v.1, HEIGHT);
    }
}

fn main() {
    let start = Instant::now();
    let mut info = Vec::new();
    let file = fs::read_to_string("src/files/physiscs.txt").expect("Unable to read file");
    //let file = "p=0,4 v=3,-3
    //                    p=6,3 v=-1,-3
    //                    p=10,3 v=-1,2
    //                    p=2,0 v=2,-1
    //                    p=0,0 v=1,3
    //                    p=3,0 v=-2,-2
    //                    p=7,6 v=-1,-3
    //                    p=3,0 v=-1,-2
    //                    p=9,3 v=2,3
    //                    p=7,3 v=-1,2
    //                    p=2,4 v=2,-3
    //                    p=9,5 v=-3,-3".to_string();
    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    for line in file.lines() {
        println!("{:?}", line);
        let caps = re.captures(line).unwrap();

        info.push(Guard{x: caps[1].parse::<i32>().unwrap(), y: caps[2].parse::<i32>().unwrap(), v: (caps[3].parse::<i32>().unwrap(), caps[4].parse::<i32>().unwrap())});
    }
    println!("{:?}", info);
    let x = sec(info.clone(), 100);
    println!("{:?}", x);
    let s = split_cont(x);
    println!("{:?}", s);
    println!("{:?}", start.elapsed());
    // using CRT seems like a smart idea that im totally stealing!
    let c = part2(info);
    println!("{:?}", c);
    println!("{:?}", start.elapsed());
    // today is so cahocitc but so cool
}

fn part2(info: Vec<Guard>) -> i32 {
    let mut info = info;
    let mut current_min_x = i32::MAX;
    let mut current_min_y = i32::MAX;
    let mut min_tick_x = 0;
    let mut min_tick_y = 0;

    for x in 0..WIDTH {
        let mut avg_d_from_ctr = 0;

        for g in &mut info{
            g.sx();
            avg_d_from_ctr += (g.x - WIDTH/2).pow(2);
        }
        if avg_d_from_ctr < current_min_x {
            current_min_x = avg_d_from_ctr;
            min_tick_x = x + 1;
        }
    }
    for y in 0..HEIGHT {
        let mut avg_d_from_ctr = 0;

        for g in &mut info {
            g.sy();
            avg_d_from_ctr += (g.y - WIDTH/2).pow(2);
        }
        if avg_d_from_ctr < current_min_y {
            current_min_y = avg_d_from_ctr;
            min_tick_y = y + 1;
        }
    }

    let result = min_tick_x + ((modular_inverse(WIDTH, HEIGHT) * (min_tick_y - min_tick_x)) % HEIGHT) * WIDTH;

    result
}
fn split_cont(info: Vec<Guard>) -> i32 {
    const HWIDTH: i32 = WIDTH/2;
    const HHEIGHT: i32 = HEIGHT/2;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;
    let mut c4 = 0;
    for g in info {
        // 2 | 1
        // 3 | 4
        if g.x > HWIDTH && g.y > HHEIGHT {
            c1 += 1;
        }
        else if g.x < HWIDTH && g.y > HHEIGHT {
            c2 += 1;
        }
        else if g.x < HWIDTH && g.y < HHEIGHT {
            c3 += 1;
        }
        else if g.x > HWIDTH && g.y < HHEIGHT {
            c4 += 1;
        }
    }
    c1*c2*c3*c4
}   

fn sec(i: Vec<Guard>, secs: i32) -> Vec<Guard> {
    let mut i = i;
    for g in &mut i {
        g.sec(secs);
    }
    i
}



fn modular_inverse(a: i32, m: i32) -> i32 {
    // Extended Euclidean Algorithm to find modular inverse
    let mut m0 = m;
    let mut x0 = 0;
    let mut x1 = 1;
    let mut a = a;

    while a > 1 {
        let q = a / m0;
        let t = m0;

        m0 = a % m0;
        a = t;

        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    x1
}
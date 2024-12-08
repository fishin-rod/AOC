use std::time::Instant;
use std::fs;

//Stole somthing I understand from Sam
fn main() {
    let start = Instant::now();
    let file_content = fs::read_to_string("src/files/xmas.txt").expect("Unable to read file");

    let found_words = find_words(file_content.as_str());
    let p2 = f2(file_content.as_str());

    println!("Found {} words", found_words);
    println!("Found {} words in part 2", p2);

    println!("{:?}", start.elapsed());
}

fn find_words(grid: &str) -> i32 {
        let g: Vec<Vec<_>> = grid.lines().map(|l| l.bytes().collect()).collect();
        let mut c = 0;
        let p = [b'X', b'M', b'A', b'S', b'.'];
        for y in 1..=g.len() {
            for x in 1..=g[0].len() {
                if g[y - 1][x - 1] == b'X' {
                    for (dx, dy) in [
                        (0, 1),
                        (1, 0),
                        (0, -1),
                        (-1, 0),
                        (-1, -1),
                        (1, 1),
                        (-1, 1),
                        (1, -1),
                    ] {
                        let mut j = 0;
                        let (mut xx, mut yy) = (x, y);
                        while j < 4
                            && yy > 0
                            && yy <= g.len()
                            && xx > 0
                            && xx <= g[0].len()
                            && g[yy - 1][xx - 1] == p[j]
                        {
                            xx = (xx as i32 + dx) as usize;
                            yy = (yy as i32 + dy) as usize;
                            j += 1
                        }
                        if j > 3 {
                            c += 1
                        }
                    }
                }
            }
        }
        c as i32
}

fn f2(s: &str) -> usize {
    let g: Vec<Vec<_>> = s.lines().map(|l| l.bytes().collect()).collect();
    let mut c = 0;
    for y in 1..g.len() - 1 {
        for x in 1..g[0].len() - 1 {
            if g[y][x] == b'A' {
                match [
                    g[y + 1][x + 1],
                    g[y - 1][x - 1],
                    g[y - 1][x + 1],
                    g[y + 1][x - 1],
                ] {
                    [b'M', b'S', b'M', b'S'] => c += 1,
                    [b'M', b'S', b'S', b'M'] => c += 1,
                    [b'S', b'M', b'M', b'S'] => c += 1,
                    [b'S', b'M', b'S', b'M'] => c += 1,
                    _ => {}
                }
            }
        }
    }
    c
}
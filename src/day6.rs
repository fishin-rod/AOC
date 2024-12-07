use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file = fs::read_to_string("src/files/map.txt").expect("Unable to read file");
    //index of 1st vec row
    // index of 2nd vec column
    let lines = file.lines().collect::<Vec<&str>>();
    let columns = lines.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut hits: Vec<(i32, i32)> = Vec::new();
    //println!("{:?}", columns);
    // get initial position
    //row, col
    let row = columns.iter().position(|x| x.contains(&'^')).unwrap();
    let mut gcoords = (row as i32, columns[row].iter().position(|x| *x == '^').unwrap() as i32);
    hits.push(gcoords);
    // totalty stealing from scott
    let dirs = [(-1,0), (0,1), (1,0), (0,-1)];
    let mut c_dir = 0;
    // Advent of code 2024: The great bool markers
    let mut ins = true;
    // this is gonna be crazy
    let mut tags: Vec<(i32, i32)> = columns.iter().enumerate().flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_index, &cell)| {
                    if cell == '#' {
                        Some((row_index as i32, col_index as i32))
                    } else {
                        None
                    }
                })
        })
        .collect();
    println!("{:?}", tags);
    tags.remove(tags.iter().position(|(x,y)| (x,y)==(&52,&118)).unwrap());
    while ins {
        if gcoords.0 >= 130 || gcoords.1 >= 130 {
            ins = false
        }
        if tags.contains(&(gcoords.0 + dirs[c_dir].0, gcoords.1)) {
            if c_dir == 3 {
                c_dir = 0;
            }
            else {
                c_dir += 1;
            }
            continue;
        }
        else {
            // this may or may not lead to problems
            gcoords.0 += dirs[c_dir].0;
            gcoords.1 += dirs[c_dir].1;
            hits.push(gcoords);
            println!("{:?}", gcoords);
        }
    }
    hits.sort();
    hits.dedup();
    println!("{:?}", hits.len());
    println!("{:?}", start.elapsed());
}

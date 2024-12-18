// so unconfident today I havent even added the actual file
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let map = 
    "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########".to_string();
    let mut map = map.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let drs = "<^^>>>vv<v>>v<<".to_string().chars().collect::<Vec<char>>();

    let mut cur_dir = 0; // 0 = left, 1 = right, 3 = up, 4 = down
    let dirs = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut pos = (0, 0); //row, col
    pos.0 = map.iter().position(|x| x.contains(&'@')).unwrap();
    pos.1 = map[pos.0].iter().position(|x| x == &'@').unwrap();

    for ch in drs {
        match ch {
            '<' => {cur_dir = (cur_dir + 1) % 4;},
            '>' => {cur_dir = (cur_dir + 3) % 4;},
            '^' => {cur_dir = (cur_dir + 2) % 4;},
            'v' => {cur_dir = (cur_dir + 4) % 4;},
            _ => {panic!("WHAT")}
        }
        map = movenpush(dirs[cur_dir], pos, map.clone());
    }
    println!("{:?}", map);
    println!("{:?}", start.elapsed());
}

fn movenpush(dir: (i32, i32), pos: (usize, usize), map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map;
    let new_row = pos.0 as i32 + dir.0;
    let new_col = pos.1 as i32 + dir.1;
    let new_row = new_row as usize;
    let new_col = new_col as usize;
    println!("{:?}, {:?}, {:?}", map.len(), dir, pos);
    if map[new_row][new_col] == '#' {return map;}
    else if map[new_row][new_col] == '.' {
        map[pos.0 as usize][pos.1 as usize] = '.';
        map[new_row][new_col] = '@';
        return map;}
    else {
        // kinda stealing this but its way differnt in reality
        let mut mov = false;
        let mut n = 1;
        while mov == false {
            if map[(pos.0 as i32 + (dir.0 * n)) as usize][(pos.1 as i32 + (dir.1 * n)) as usize] == 'O' {
                n+=1;
            }
            else if map[(pos.0 as i32 + (dir.0 * n)) as usize][(pos.1 as i32 + (dir.1 * n)) as usize] == '#' {
                return map;
            }
            else {
                mov = true;
            }
        }
        map[pos.0 as usize][pos.1 as usize] = '.';
        map[new_row][new_col] = '@';
        for i in 1..n {
            map[(pos.0 as i32 + (dir.0 * i)) as usize][(pos.1 as i32 + (dir.1 * i)) as usize] = 'O';
        }
    }
    map
}
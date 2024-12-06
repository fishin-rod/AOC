use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    let file_content = fs::read_to_string("src/files/xmas.txt").expect("Unable to read file");
    let grid: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    println!("{:?}", grid);
    let found_words = find_words(&grid);

    for word in found_words.clone() {
        println!("Found: {}", word);
    }
    println!("Found {} words", found_words.clone().len());

    println!("{:?}", start.elapsed());
}

fn find_words(grid: &[Vec<char>]) -> Vec<String> {
    let mut found_words = Vec::new();
    // Search horizontally
    for row in grid {
        let horizontal: String = row.iter().collect();
        if horizontal.contains("xmas") {
            found_words.push("xmas".to_string());

        }
    }
    // Search vertically
    for col in 0..grid[0].len() {
        let vertical: String = grid.iter().map(|row| row[col]).collect();
        if vertical.contains("xmas") {
            found_words.push("xmas".to_string());
        }
    }

    // Search diagonally (down-right and down-left)
    let rows = grid.len();
    let cols = grid[0].len();
/* 
    // Down-right diagonals
    for start in 0..(rows + cols - 1) {
        let mut diagonal = String::new();
        for i in 0..=start {
            let row = i;
            let col = start - i;
            if row < rows && col < cols {
                diagonal.push(grid[row][col]);
            }
        }
        if diagonal.contains("xmas") {
            found_words.push("xmas".to_string());
        }
    }
 
    // Down-left diagonals
    for start in 0..(rows + cols - 1) {
        let mut diagonal = String::new();
        for i in 0..=start {
            let row = i;
            let col = cols - 1 - (start - i);
            if row < rows && col < cols {
                diagonal.push(grid[row][col]);
            }
        }
        if diagonal.contains("xmas") {
            found_words.push("xmas".to_string());
        }
    }
    */

    found_words
}

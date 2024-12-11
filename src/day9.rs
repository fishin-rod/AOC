use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    //let file = "2333133121414131402".to_string();
    let file = fs::read_to_string("src/files/files.txt").expect("Unable to read file").replace("\r\n", "");
    println!("{:?}", file);
    let mut fin = String::new();
    for i in (0..file.chars().count()).step_by(2) {
        if i >= file.chars().count() - 2 {
            fin.push_str(add_blocks((file.as_bytes()[file.chars().count()-1] - 48) as i32, (i/2) as i32).as_str());
        }
        else {
            fin.push_str(add_blocks((file.as_bytes()[i] - 48) as i32, (i/2) as i32).as_str());// call me a genius or an idiot but it might work
            fin.push_str(add_spaces((file.as_bytes()[i+1] - 48) as i32).as_str());  
        }
    }
    //981283324507
    //89558806610
    //83676789979
    // to low
    let fin = mov2(fin);
    //println!("{:?}", fin);
    let mut sum = 0;
    // IDS CAN HAVE MUTLIPLE BYTES NEED TO MULTIPLY BY 2 digit #s
    for i in 0..fin.chars().count() {
        println!("{:?}", fin.as_bytes()[i]-48);
        sum += i * (fin.as_bytes()[i]-48) as usize;
    }
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}

fn mov2(mut input: String) -> String {
    while input.contains('.') {
        if let Some(dot_index) = input.find('.') {
            if let Some(last_char) = input.chars().last() {
                input.replace_range(dot_index..dot_index + 1, &last_char.to_string());
                input.truncate(input.len() - 1);
            }
        }
    }
    input
}

// I really really wanted this to work but alas it didnt
fn mov(s: String) -> String {
    let mut st = s;
    // move from right to first availive dot then add remove dot
    // I am a smart one today
    // When did I become good at coding?
    let dot_idx: Vec<usize> = st.chars().enumerate().filter_map(|x| if x.1 == '.' {Some(x.0)} else {None}).collect();
    let mut chars = st.chars().collect::<Vec<char>>();
    let chars_idx: Vec<usize> = st.chars().enumerate().filter_map(|x| if x.1 == '.' {None} else {Some(x.0)}).collect();
    chars.retain(|x| *x != '.');
    for i in 0..dot_idx.len()-1 {
        st.insert(dot_idx[i].clone(), chars[chars.len()-1-i]);
        println!("{:?}", chars.len()-1-i);
        st.pop();
        st.remove(dot_idx[i]+1);
    }
    for i in 0..dot_idx.len() {
        st.push('.');
    }
    st
}

fn add_blocks(n: i32, i : i32) -> String {
    let mut st = String::new();
    
        for _ in 0..n {
            // I spoke to soon mb
            st.push_str(i.to_string().as_str());
        }
    st
}

fn add_spaces(n: i32) -> String {
    let mut st = String::new();
    for _ in 0..n {
        st.push('.');
    }
    st
}
fn main() {
    let pats = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    let des = vec![
        "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",
    ];
    let mut sum = 0;
    //const MAXLEN: i32 = 3;
    for word in des {
        let mut idx = 0;
        let mut patss = Vec::new();
        let chars: Vec<char> = word.as_bytes().iter().map(|x| *x as char).collect();
        let mut pat = 0;
        while pat < word.chars().count() {
            let pchars: Vec<char> = pats[pat].as_bytes().iter().map(|x| *x as char).collect();
            //println!("{:?}..{:?}", pchars, chars[idx]);
            let ch = &chars[idx..pchars.len()];
            println!("{:?}", ch);
            if &chars[idx..pchars.len()] == pchars.as_slice() {
                patss.push(pats[pat]);
                println!("w: {:?}, p: {:?}", word, chars[idx]);
                idx += pchars.len();
                println!("{:?}", idx);
                //println!("w: {:?}, p: {:?}", word, chars[idx]);
                pat = 0;
            }
            pat += 1;
        }
    }
}
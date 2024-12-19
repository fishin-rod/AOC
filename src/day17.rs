use std::time::Instant;

struct Comp {
    RegA: i128,
    RegB: i128,
    RegC: i128,
    Codes: Vec<i128>,
    Opcode: usize,
    Oper: i128,
    Out: Vec<i128>,
}

fn main() {
    let start = Instant::now();
    //let a = 30344604;
    let a = 117440;
    let b = 0;
    let c = 0;
    //let codes = vec![0,3,5,4,3,0];
    let codes = vec![2,4,1,1,7,5,1,5,4,5,0,3,5,5,3,0];
    let mut x = 0;
    let mut nums = 1;
    loop {
        // from 158088157855745 to 
        //    
        //      164545992421053 mine
        // why is mine like 10000 higher
        //      164540892147389 online
        // scott if you seeing this tell me why
        //     
        let a = x;
        let comp = Comp::new(a, b, c, codes.clone()).run();
        if comp == codes {
            println!("ans{:?}", a);
            println!("comp: {:?}, codes: {:?}", comp, codes);
            break;
        }
        if comp[comp.len() - nums..] == codes[codes.len() - nums..] {
            println!("comp: {:?}, codes: {:?}", comp, codes);
            nums += 1;
            x *= 8;
        }
        x+=1 
    }
    //let _out  = Comp::new(a, b, c, codes).run();
    println!("{:?}", start.elapsed());
}

impl Comp {
    fn new(a: i128, b: i128, c: i128, codes: Vec<i128>) -> Comp {
        Comp {
            RegA: a,
            RegB: b,
            RegC: c,
            Codes: codes,
            Opcode: 0,
            Oper: 0,
            Out: Vec::new(),
        }
    }

    fn run(&mut self) -> Vec<i128> {
        // iterate throught the list of codes
        // send either the literal or commbo code to Oper
        // my color scheme is making this look sexy
        while self.Opcode < self.Codes.len() {
            match self.Codes[self.Opcode] {
                0 => {self.Oper = self.combo(self.Codes[self.Opcode+1]); self.adv();},
                1 => {self.Oper = self.Codes[self.Opcode+1]; self.bxl();},
                2 => {self.Oper = self.combo(self.Codes[self.Opcode+1]); self.bst();},
                3 => {self.Oper = self.Codes[self.Opcode+1]; self.jnz();},
                4 => {self.bxc();},
                5 => {self.Oper = self.combo(self.Codes[self.Opcode+1]); self.out();},
                6 => {self.Oper = self.combo(self.Codes[self.Opcode+1]); self.bdv();},
                7 => {self.Oper = self.combo(self.Codes[self.Opcode+1]); self.cdv();},
                _ => {panic!("AEYYEYEYE")},
            }
        }
        //println!("{:?}", self.Out);
        self.Out.clone()
    }

    fn combo(&mut self, op: i128) -> i128 {
        match op {
            0..=3 => op,
            4 => self.RegA,
            5 => self.RegB,
            6 => self.RegC,
            7 => panic!("a"),
            _ => panic!("a"),
        }
    }

    
    fn adv(&mut self) -> &mut Self {
        // combo
        // attempt to multiply with overflow :(
        //println!("{:?}", self.Oper);
        let num = self.RegA as f64 / 2_i128.pow(self.Oper as u32) as f64;
        self.RegA = num.floor() as i128;
        self.Opcode += 2;
        self
    }

    fn bxl(&mut self) -> &mut Self {
        // literal  
        self.RegB = self.RegB ^ self.Oper;
        self.Opcode += 2;
        self
    }

    fn bst(&mut self) -> &mut Self {
        // combo
        self.RegB = self.Oper % 8;
        self.Opcode += 2;
        self
    }

    fn jnz(&mut self) -> &mut Self {
        // literal
        if self.RegA != 0 {
            self.Opcode = self.Oper as usize;
        }
        else {
            self.Opcode += 2;
        }
        self
    }

    fn bxc(&mut self) -> &mut Self {
        // literal but ignored
        self.RegB = self.RegB ^ self.RegC;
        self.Opcode += 2;
        self
    }

    fn out(&mut self) -> &mut Self {
        // combo
        self.Out.push(self.Oper % 8);
        self.Opcode += 2;
        self
    }

    fn bdv(&mut self) -> &mut Self {
        // combo
        let num = self.RegA as f64 /2_i128.pow(self.Oper as u32) as f64;
        self.RegB = num.floor() as i128;
        self.Opcode += 2;
        self
    }

    fn cdv(&mut self) -> &mut Self {
        // combo
        let num = self.RegA as f64 /2_i128.pow(self.Oper as u32) as f64;
        self.RegC = num.floor() as i128;
        self.Opcode += 2;
        self
    }
}
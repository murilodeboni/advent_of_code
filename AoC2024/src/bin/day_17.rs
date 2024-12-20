use std::time::Instant;

#[derive(Debug)]
struct Chrono {
    A: usize,
    B: usize,
    C: usize, 
    Program: Vec<usize>,
    instruction: usize,
    Ans: Vec<usize>,
    halted: bool
} 

impl Chrono {
    fn reset(&mut self, i: usize) {
        self.A = i;
        self.B = 0;
        self.C = 0;
        self.Program = vec![0,3,5,4,3,0];
        self.instruction = 0;
        self.Ans = Vec::new();
        self.halted = false;
    }
    
    fn inc_pointer(&mut self) {
        self.instruction += 2;
    }
    
    fn run(&mut self) {
        if self.instruction >= self.Program.len() {
            self.halted = true;
        } else {
            let opcode = self.Program[self.instruction];
            let op = self.Program[self.instruction+1];
            match opcode {
                0 => self.adv(op),
                1 => self.bxl(op),
                2 => self.bst(op),
                3 => self.jnz(op),
                4 => self.bxc(op),
                5 => self.out(op),
                6 => self.bdv(op),
                7 => self.cdv(op),
                _ => println!("error")
            }
        }
    }
    
    // Combo operands 0 through 3 represent literal values 0 through 3.
    // Combo operand 4 represents the value of register A.
    // Combo operand 5 represents the value of register B.
    // Combo operand 6 represents the value of register C.
    // Combo operand 7 is reserved and will not appear in valid programs.
    fn combo_op(&self, n: usize) -> usize {
        if n <= 3 {
            return n
        } else if n == 4 {
            return self.A
        } else if n == 5 {
            return self.B 
        } else if n == 6 {
            return self.C
        } else {
            0 // should not get here 
        }
    }
    
    // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. 
    // The denominator is found by raising 2 to the power of the instruction's combo operand. 
    // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) 
    // The result of the division operation is truncated to an integer and then written to the A register.
    fn adv(&mut self, op: usize) {
        self.A = (self.A as f32/(2_f32.powf(self.combo_op(op) as f32))).floor() as usize;
        self.inc_pointer();
    }
    
    //The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, 
    // then stores the result in register B.
    fn bxl(&mut self, op: usize) {
        self.B = self.B ^ op;
        self.inc_pointer();
    }
    
    // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), 
    // then writes that value to the B register.
    fn bst(&mut self, op: usize) {
        self.B = self.combo_op(op) % 8;
        self.inc_pointer();
    }
    
    // The jnz instruction (opcode 3) does nothing if the A register is 0. 
    // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; 
    // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    fn jnz(&mut self, op: usize) {
        if self.A != 0 {
            self.instruction = op;
        } else {
            self.inc_pointer();
        }
    }

    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. 
    // (For legacy reasons, this instruction reads an operand but ignores it.)
    fn bxc(&mut self, op: usize) {
        self.B = self.B ^ self.C;
        self.inc_pointer();
    }

    // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. 
    // (If a program outputs multiple values, they are separated by commas.)
    fn out(&mut self, op: usize) {
        self.Ans.push(self.combo_op(op) % 8);
        self.inc_pointer();
    }

    // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. 
    // (The numerator is still read from the A register.)
    fn bdv(&mut self, op: usize) {
        self.A = (self.A as f32/(2_f32.powf(self.combo_op(op) as f32))).floor() as usize;
        self.inc_pointer();
    }
    
    // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. 
    // (The numerator is still read from the A register.)
    fn cdv(&mut self, op: usize) {
        self.C = (self.A as f32/(2_f32.powf(self.combo_op(op) as f32))).floor() as usize;
        self.inc_pointer();
    }

}

fn main() {
    let start = Instant::now();

    // part 1
    // while !crono.halted {
    //     crono.run();
    // }

    let mut i = 2024;
    let mut crono = Chrono {
        A: i,
        B: 0,
        C: 0,
        Program: vec![0,3,5,4,3,0],
        instruction: 0,
        halted: false,
        Ans: Vec::new()
    };

    while crono.Ans != crono.Program {

        i += 1;
        crono.reset(i);
        while !crono.halted {
            crono.run();
        }
        println!("trying {:?} {}", crono, i);
    }


    println!("{:?}", crono.A)
}

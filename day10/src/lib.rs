use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
pub enum Instruction {
    #[from_str(regex = "noop")]
    Noop,
    #[from_str(regex = "addx (?P<0>-?[0-9]+)")]
    Add(isize),
}

impl Instruction {
    pub fn cycle(&self) -> usize {
        match self {
            Instruction::Noop => 0,
            Instruction::Add(_) => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cpu {
    pc: usize,
    pub x: isize,
    mem: Vec<Instruction>,
    pub cycle: usize,
    waiting_for: usize,

    finished: usize,
}

impl Cpu {
    pub fn new(instructions: Vec<Instruction>) -> Cpu {
        Cpu {
            pc: 0,
            x: 1,
            cycle: 1,
            waiting_for: instructions[0].cycle(),
            mem: instructions,
            finished: 0,
        }
    }

    pub fn process_cycle(&mut self) {
        if self.finished != 0 {
            self.finished += 1;
            return;
        }
        self.cycle += 1;

        if self.waiting_for != 0 {
            self.waiting_for -= 1;
        } else {
            self.execute();
            self.pc += 1;
            if let Some(instruction) = self.mem.get(self.pc) {
                self.waiting_for = instruction.cycle();
            } else {
                self.finished = 1;
            }
        }
    }

    pub fn signal(&self) -> isize {
        self.x * self.cycle as isize
    }

    pub fn finished(&self) -> bool {
        self.finished == 2
    }

    fn execute(&mut self) {
        match self.mem[self.pc] {
            Instruction::Noop => (),
            Instruction::Add(x) => self.x += x,
        }
    }
}

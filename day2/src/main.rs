#![warn(clippy::all)]

#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate enum_primitive;
extern crate num;
use num::FromPrimitive;

lazy_static_include_str!(TEST, "data/input.txt");

enum_from_primitive! {
#[repr(u8)]
#[derive(Debug, PartialEq)]
enum Opcode {
    Add = 1,
    Mul = 2,
    End = 99,
}
}

#[derive(Debug)]
enum Read {
    Opcode(Opcode),
    Data(i64),
}

#[derive(Debug)]
enum MathOp {
    Add,
    Mul
}

impl MathOp {
    fn new(opcode: Opcode) -> Option<MathOp> {
        match opcode {
            Opcode::Add => Some(MathOp::Add),
            Opcode::Mul => Some(MathOp::Mul),
            _ => None
        }
    }
}


#[derive(Debug)]
struct Instruction {
    math_code: MathOp,
    input1: i64,
    input2: i64,
    output_position: i64,
}

impl Instruction {
    fn new(opcode: Opcode, computer: &Computer) -> Option<Self> {
        match MathOp::new(opcode) {
            Some(x) => Some(Instruction { 
                math_code: x,
                input1: computer.intcode[computer.intcode[computer.position + 1] as usize],
                input2: computer.intcode[computer.intcode[computer.position + 2] as usize],
                output_position: computer.intcode[computer.position + 3],
            }),
            None => None,
        }
    }

    fn calculate(&self) -> i64 {
        match self.math_code {
            MathOp::Add => self.input1 + self.input2,
            MathOp::Mul => self.input1 * self.input2,
        }
    }
}

#[derive(Debug)]
struct Computer {
    intcode: Vec<i64>,
    position: usize,
}

impl Computer {
    fn new() -> Self {
        Computer {
            intcode: Vec::new(),
            position: 0
        }
    }

    fn run(&mut self) {
        loop {
            if !self.step() {
                break;
            }
        }
    }

    fn run_load(&mut self, intcode: &Vec<i64>) -> &Vec<i64> {
        self.load(intcode);
        self.run();
        &self.intcode
    }

    fn reset(&mut self) {
        self.intcode.clear();
        self.position = 0;
    }

    fn load(&mut self, intcode: &Vec<i64>) {
        self.intcode = intcode.clone();
        self.position = 0;
    }

    /// Returns false after program halts
    fn step(&mut self) -> bool {
        match self.read() {
            Read::Opcode(opcode) => {
                if opcode == Opcode::End {
                    false
                } else if let Some(instruction) = Instruction::new(opcode, self) {
                    let result = instruction.calculate();
                    self.intcode[instruction.output_position as usize] = result;
                    self.position += 4;
                    true
                } else {
                    true
                }
            },
            Read::Data(_) => true,
        }
    }

    fn read(&self) -> Read {
        let data = self.read_raw();
        match Opcode::from_i64(data) {
            Some(opcode) => Read::Opcode(opcode),
            None => Read::Data(data)
        }
    }

    fn read_opcode(&self) -> Option<Opcode> {
        Opcode::from_i64(self.read_raw())
    }

    fn read_raw(&self) -> i64 {
        self.read_raw_at(self.position)
    }

    fn read_raw_at(&self, position: usize) -> i64 {
        self.intcode[position]
    }
}

/// https://adventofcode.com/2019/day/2
fn main() {
    let mut data = parse_data();
    let mut computer = Computer::new();
    data[1] = 12;
    data[2] = 2;
    let result = computer.run_load(&data);
    println!("Part 1: {}", result[0]);

    for i in 0..data.len() - 1 {
        for j in 0..data.len() - 1 {
            data[1] = i as i64;
            data[2] = j as i64;
            let result = computer.run_load(&data);
            if result[0] == 19_690_720 {
                println!("Part 2: [{}, {}] = {}", i, j, 100 * i + j);
            }
        }
    }
}

fn parse_data() -> Vec<i64> {
    TEST.split(',').map(|x| x.parse::<i64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = parse_data();
        assert_eq!(data[0], 1);
        assert_eq!(data[1], 0);
    }

    #[test]
    fn test_opcodes() {
        assert_eq!(Opcode::from_u8(1), Some(Opcode::Add));
        assert_eq!(Opcode::from_u8(2), Some(Opcode::Mul));
        assert_eq!(Opcode::from_u8(99), Some(Opcode::End));
        assert_eq!(Opcode::from_u8(3), None);
    }

    #[test]
    fn test_part_1() {
        let mut computer = Computer::new();
        assert_eq!(computer.run_load(&vec![1,0,0,0,99]), &vec![2,0,0,0,99]);
        assert_eq!(computer.run_load(&vec![2,3,0,3,99]), &vec![2,3,0,6,99]);
        assert_eq!(computer.run_load(&vec![2,4,4,5,99,0]), &vec![2,4,4,5,99,9801]);
        assert_eq!(computer.run_load(&vec![1,1,1,4,99,5,6,0,99]), &vec![30,1,1,4,2,5,6,0,99]);
    }
}

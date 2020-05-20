use std::io::{self, Read};

const MAX_MEM: usize = 64 * 1024;

#[derive(PartialEq)]
enum Ops {
    INP,
    DEP,
    INC,
    DEC,
    PRN,
    RDC,
    BRZ(usize),
    RET(usize),
    NOP,
}

fn main() -> io::Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let intermediate_program: Vec<&u8> = buffer
        .iter()
        .filter(|b| match **b {
            b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']' => true,
            _ => false,
        })
        .collect();

    let program: Vec<Ops> = intermediate_program
        .iter()
        .enumerate()
        .map(|(cur_pc, b)| match *b {
            b'>' => Ops::INP,
            b'<' => Ops::DEP,
            b'+' => Ops::INC,
            b'-' => Ops::DEC,
            b'.' => Ops::PRN,
            b',' => Ops::RDC,
            b'[' => {
                let mut pc = cur_pc;
                let mut loop_count = 1;
                while loop_count != 0 {
                    pc += 1;
                    if *intermediate_program[pc] == b'[' {
                        loop_count += 1;
                    } else if *intermediate_program[pc] == b']' {
                        loop_count -= 1;
                    }
                }
                Ops::BRZ(pc + 1)
            },
            b']' => {
                let mut pc = cur_pc;
                let mut loop_count = 1;
                while loop_count != 0 {
                    pc -= 1;
                    if *intermediate_program[pc] == b']' {
                        loop_count += 1;
                    } else if *intermediate_program[pc] == b'[' {
                        loop_count -= 1;
                    }
                }
                Ops::RET(pc)
            },
            _ => Ops::NOP,
        })
        .collect();

    let program_size = program.len();
    if program_size != 0 {
        println!("Starting program in 3.. 2.. 1.. Now!");

        let mut memory: [u8; MAX_MEM] = [0; MAX_MEM];
        let mut pc: usize = 0;
        let mut ptr: usize = 0;

        while pc < program_size {
            match program[pc] {
                Ops::INP => {
                    ptr = if ptr != MAX_MEM { ptr + 1 } else { 0 };
                    pc += 1;
                }
                Ops::DEP => {
                    ptr = if ptr != 0 { ptr - 1 } else { MAX_MEM };
                    pc += 1;
                }
                Ops::INC => {
                    memory[ptr] = memory[ptr].wrapping_add(1);
                    pc += 1;
                }
                Ops::DEC => {
                    memory[ptr] = memory[ptr].wrapping_sub(1);
                    pc += 1;
                }
                Ops::PRN => {
                    print!("{}", memory[ptr] as char);
                    pc += 1;
                }
                Ops::RDC => {
                    /* FIXME: read char */
                    pc += 1;
                }
                Ops::BRZ(addr) => {
                    if memory[ptr] == 0 {
                        pc = addr;
                    } else {
                        pc += 1
                    }
                }
                Ops::RET(addr) => {
                    pc = addr;
                }
                Ops::NOP => (),
            }
        }
    }

    Ok(())
}

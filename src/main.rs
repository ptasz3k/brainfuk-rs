use std::io::{self, Read};

const MAX_MEM: usize = 64 * 1024;

#[derive(Clone, Debug)]
enum Tok {
    INP,
    DEP,
    INC,
    DEC,
    PRN,
    RDC,
    BRZ,
    RET,
}

#[derive(Debug)]
enum Op {
    INP,
    DEP,
    INC,
    DEC,
    PRN,
    RDC,
    LOOP(Vec<Op>),
}

fn run(ops: &Vec<Op>, mem: &mut [u8], ptr: &mut usize) {
    for op in ops {
        match op {
            Op::INP => *ptr += 1,
            Op::DEP => *ptr -= 1,
            Op::INC => mem[*ptr] = mem[*ptr].wrapping_add(1),
            Op::DEC => mem[*ptr] = mem[*ptr].wrapping_sub(1),
            Op::PRN => print!("{}", mem[*ptr] as char),
            Op::RDC => { /* FIXME: read char */ }
            Op::LOOP(inner) => {
                while mem[*ptr] != 0 {
                    run(&inner, mem, ptr)
                }
            }
        }
    }
}

fn tokenize(input: &str) -> Vec<Tok> {
    input
        .chars()
        .filter_map(|b| match b {
            '>' => Some(Tok::INP),
            '<' => Some(Tok::DEP),
            '+' => Some(Tok::INC),
            '-' => Some(Tok::DEC),
            '.' => Some(Tok::PRN),
            ',' => Some(Tok::RDC),
            '[' => Some(Tok::BRZ),
            ']' => Some(Tok::RET),
            _ => None,
        })
        .collect()
}

fn parse(tokens: &Vec<Tok>) -> Vec<Op> {
    let mut program: Vec<Op> = Vec::new();

    let mut loop_end = 0;
    tokens.iter().enumerate().for_each(|(i, t)| {
        if i >= loop_end {
            match t {
                Tok::INP => program.push(Op::INP),
                Tok::DEP => program.push(Op::DEP),
                Tok::INC => program.push(Op::INC),
                Tok::DEC => program.push(Op::DEC),
                Tok::PRN => program.push(Op::PRN),
                Tok::RDC => program.push(Op::RDC),
                Tok::BRZ => {
                    let mut pc = i;
                    let mut loop_count = 1;
                    while loop_count != 0 {
                        pc += 1;
                        match tokens[pc] {
                            Tok::BRZ => loop_count += 1,
                            Tok::RET => loop_count -= 1,
                            _ => (),
                        }
                    }
                    loop_end = pc;
                    program.push(Op::LOOP(parse(&tokens[i + 1..pc].to_vec())))
                }
                _ => (),
            }
        }
    });

    program
}

fn main() -> io::Result<()> {
    let mut input: String = String::from("");
    io::stdin().read_to_string(&mut input)?;

    let tokens = tokenize(&input);
    let program = parse(&tokens);

    // println!("{:?}", tokens);
    // println!("{:?}", program);

    println!("Starting program in 3.. 2.. 1.. Now!");
    let mut memory: [u8; MAX_MEM] = [0; MAX_MEM];
    let mut ptr: usize = 0;
    run(&program, &mut memory, &mut ptr);

    Ok(())
}

use std::io::{self, Read};

const MAX_MEM: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    INP,
    DEP,
    INC,
    DEC,
    PRN,
    RDC,
    BRZ,
    RET,
    NOP,
}

#[derive(Debug)]
enum Op {
    INP(usize),
    DEP(usize),
    INC(u8),
    DEC(u8),
    PRN,
    RDC,
    LOOP(Vec<Op>),
}

fn run(ops: &Vec<Op>, mem: &mut [u8], ptr: &mut usize) {
    for op in ops {
        match op {
            Op::INP(n) => *ptr += n,
            Op::DEP(n) => *ptr -= n,
            Op::INC(n) => mem[*ptr] = mem[*ptr].wrapping_add(*n),
            Op::DEC(n) => mem[*ptr] = mem[*ptr].wrapping_sub(*n),
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
    let mut tokens: Vec<Tok> = input
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
        .collect();

    tokens.push(Tok::NOP);
    tokens
}

fn parse(tokens: &Vec<Tok>) -> Vec<Op> {
    let mut program: Vec<Op> = Vec::new();

    let mut loop_end = 0;
    tokens.iter().enumerate().for_each(|(i, t)| {
        if i >= loop_end {
            match t {
                Tok::INP => program.push(Op::INP(1)),
                Tok::DEP => program.push(Op::DEP(1)),
                Tok::INC => program.push(Op::INC(1)),
                Tok::DEC => program.push(Op::DEC(1)),
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

fn optimizing_parse(tokens: &Vec<Tok>) -> Vec<Op> {
    let mut program: Vec<Op> = Vec::new();
    let mut loop_end = 0;
    let mut count = 1;
    let mut prev_tok: &Tok = &Tok::NOP;
    tokens.iter().enumerate().for_each(|(i, t)| {
        if i >= loop_end {
            match prev_tok {
                Tok::INP if t == &Tok::INP => count += 1,
                Tok::INP => {
                    program.push(Op::INP(count));
                    count = 1;
                }
                Tok::DEP if t == &Tok::DEP => count += 1,
                Tok::DEP => {
                    program.push(Op::DEP(count));
                    count = 1;
                }
                Tok::INC if t == &Tok::INC => count += 1,
                Tok::INC => {
                    program.push(Op::INC((count % 256) as u8));
                    count = 1;
                }
                Tok::DEC if t == &Tok::DEC => count += 1,
                Tok::DEC => {
                    program.push(Op::DEC((count % 256) as u8));
                    count = 1;
                }
                Tok::PRN => program.push(Op::PRN),
                Tok::RDC => program.push(Op::RDC),
                Tok::BRZ => {
                    let mut pc = i - 1;
                    let mut loop_count = 1;
                    while loop_count != 0 {
                        pc += 1;
                        match tokens[pc] {
                            Tok::BRZ => loop_count += 1,
                            Tok::RET => loop_count -= 1,
                            _ => (),
                        }
                    }
                    loop_end = pc + 1;
                    let mut loop_body = tokens[i..pc].to_vec();
                    loop_body.push(Tok::NOP);
                    count = 1;
                    program.push(Op::LOOP(optimizing_parse(&loop_body)))
                }
                _ => (),
            }
        }
        prev_tok = t;
    });

    program
}

fn main() -> io::Result<()> {
    let mut input: String = String::from("");
    io::stdin().read_to_string(&mut input)?;

    let optimize = true;

    let tokens = tokenize(&input);
    let program = if optimize {
        optimizing_parse(&tokens)
    } else {
        parse(&tokens)
    };

 //   println!("{:?}", tokens);
 //   println!("{:?}", program);

    println!("Starting program in 3.. 2.. 1.. Now!");
    let mut memory: [u8; MAX_MEM] = [0; MAX_MEM];
    let mut ptr: usize = 0;
    run(&program, &mut memory, &mut ptr);

    Ok(())
}

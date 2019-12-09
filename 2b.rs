use std::io;
use std::io::prelude::*;
use std::error::Error;

fn interpret(mem: &mut [u32]) -> &mut [u32] {
    let mut pos = 0;
    while mem[pos] != 99 {
        let op1 = mem[mem[pos + 1] as usize];
        let op2 = mem[mem[pos + 2] as usize];
        let ind_result = mem[pos + 3] as usize;
        match mem[pos] {
            1 => mem[ind_result] = op1.checked_add(op2)
                .expect(&format!("overflow {}+{}", op1, op2)),
            2 => mem[ind_result] = op1.checked_mul(op2)
                .expect(&format!("overflow {}*{}", op1, op2)),
            other => panic!("invalid opcode {}", other),
        };
        pos += 4;
    }
    mem
}

fn read_prog(input: impl BufRead) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut ret = vec![];
    for line in input.lines() {
        let line = line?;
        for tok in line.split(",") {
            ret.push(tok.trim().parse()?);
        }
    }
    Ok(ret)
}

fn main() -> Result<(), Box<dyn Error>> {
    let prog = read_prog(io::stdin().lock())?;
    'outer: for noun in 0..prog.len()-1 {
        for verb in 0..prog.len()-1 {
            let mut prog = prog.clone();
            prog[1] = noun as u32;
            prog[2] = verb as u32;
            interpret(&mut prog);
            if prog[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}

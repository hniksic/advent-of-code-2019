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
    assert_eq!(interpret(&mut [1,9,10,3,2,3,11,0,99,30,40,50]),
               &mut [3500,9,10,70,2,3,11,0,99,30,40,50]);
    assert_eq!(interpret(&mut [1,0,0,0,99]), &mut [2,0,0,0,99]);
    assert_eq!(interpret(&mut [2,3,0,3,99]), &mut [2,3,0,6,99]);
    assert_eq!(interpret(&mut [2,4,4,5,99,0]), &mut [2,4,4,5,99,9801]);
    assert_eq!(interpret(&mut [1,1,1,4,99,5,6,0,99]), &mut [30,1,1,4,2,5,6,0,99]);

    let mut prog = read_prog(io::stdin().lock())?;
    prog[1] = 12;
    prog[2] = 2;
    interpret(&mut prog);
    println!("{}", prog[0]);

    Ok(())
}

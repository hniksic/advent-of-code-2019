use std::io;
use std::io::prelude::*;
use std::error::Error;

fn interpret<'a>(mem: &'a mut [i32],
                 read_input: &mut dyn FnMut() -> i32,
                 write_output: &mut dyn FnMut(i32)) -> &'a mut [i32] {
    fn mem_read(mem: &[i32], pos: usize, is_immediate: bool) -> i32 {
        let mut val = mem[pos];
        if !is_immediate {
            val = mem[val as usize];
        }
        val
    }

    fn decode_args_2(mem: &mut [i32], pos: usize) -> (i32, i32) {
        let mut opcode = mem[pos] / 100;
        let immediate_1 = opcode % 10 == 1;
        opcode /= 10;
        let immediate_2 = opcode % 10 == 1;
        (mem_read(mem, pos + 1, immediate_1),
         mem_read(mem, pos + 2, immediate_2))
    }

    fn decode_args_1(mem: &mut [i32], pos: usize) -> i32 {
        let opcode = mem[pos] / 100;
        let immediate_1 = opcode % 10 == 1;
        mem_read(mem, pos + 1, immediate_1)
    }

    let mut pos = 0;

    while mem[pos] != 99 {
        match mem[pos] % 100 {
            1 => {
                let (op1, op2) = decode_args_2(mem, pos);
                mem[mem[pos + 3] as usize] = op1.checked_add(op2)
                    .expect(&format!("overflow {}+{}", op1, op2));
                pos += 4;
            }
            2 => {
                let (op1, op2) = decode_args_2(mem, pos);
                mem[mem[pos + 3] as usize] = op1.checked_mul(op2)
                    .expect(&format!("overflow {}*{}", op1, op2));
                pos += 4;
            }
            3 => {
                mem[mem[pos + 1] as usize] = read_input();
                pos += 2;
            }
            4 => {
                write_output(decode_args_1(mem, pos));
                pos += 2;
            }
            other => panic!("invalid instruction {}", other),
        };
    }
    mem
}


fn read_prog(input: impl BufRead) -> Result<Vec<i32>, Box<dyn Error>> {
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
    let mut prog = read_prog(io::stdin().lock())?;
    interpret(&mut prog[..], &mut || 1, &mut |val| println!("{}", val));
    Ok(())
}

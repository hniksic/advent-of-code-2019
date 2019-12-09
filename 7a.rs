use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashSet;
use std::iter::FromIterator;

fn interpret(mem: &mut [i32],
             read_input: &mut dyn FnMut() -> i32) -> Vec<i32> {
    let mut output = vec![];

    fn mem_read(mem: &[i32], pos: usize, is_immediate: bool) -> i32 {
        let mut val = mem[pos];
        if !is_immediate {
            val = mem[val as usize];
        }
        val
    }

    fn decode_args_1(mem: &mut [i32], pos: usize) -> i32 {
        let opcode = mem[pos] / 100;
        let immediate_1 = opcode % 10 == 1;
        mem_read(mem, pos + 1, immediate_1)
    }

    fn decode_args_2(mem: &mut [i32], pos: usize) -> (i32, i32) {
        let mut opcode = mem[pos] / 100;
        let immediate_1 = opcode % 10 == 1;
        opcode /= 10;
        let immediate_2 = opcode % 10 == 1;
        (mem_read(mem, pos + 1, immediate_1),
         mem_read(mem, pos + 2, immediate_2))
    }

    let mut pos = 0;

    while mem[pos] != 99 {
        //println!("decoding {} at position {}", mem[pos], pos);
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
                let val = decode_args_1(mem, pos);
                output.push(val);
                pos += 2;
            }
            5 => {
                let (op1, op2) = decode_args_2(mem, pos);
                if op1 != 0 {
                    pos = op2 as usize;
                }
                else {
                    pos += 3;
                }
            }
            6 => {
                let (op1, op2) = decode_args_2(mem, pos);
                if op1 == 0 {
                    pos = op2 as usize;
                }
                else {
                    pos += 3;
                }
            }
            7 => {
                let (op1, op2) = decode_args_2(mem, pos);
                mem[mem[pos + 3] as usize] = if op1 < op2 {1} else {0};
                pos += 4;
            }
            8 => {
                let (op1, op2) = decode_args_2(mem, pos);
                mem[mem[pos + 3] as usize] = if op1 == op2 {1} else {0};
                pos += 4;
            }
            other => panic!("invalid instruction {}", other),
        };
    }
    output
}

fn amplify(program: &[i32], phase_settings: [i32; 5]) -> i32 {
    let mut input = 0;
    for &phase in &phase_settings {
        let mut running_program = program.to_vec();
        let mut first = true;
        let output = interpret(
            &mut running_program,
            &mut || if first {first = false; phase} else {input});
        assert!(output.len() == 1);
        input = output[0];
    }
    input
}

fn read_prog(input: impl BufRead) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut ret = vec![];
    for line in input.lines() {
        ret.extend(read_prog_from(&line?)?);
    }
    Ok(ret)
}

fn read_prog_from(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut ret = vec![];
    for tok in input.split(",") {
        ret.push(tok.trim().parse()?);
    }
    Ok(ret)
}

fn run_tests() -> Result<(), Box<dyn Error>> {
    assert_eq!(amplify(&read_prog_from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")?,
                       [4, 3, 2, 1, 0]),
               43210);
    assert_eq!(amplify(&read_prog_from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")?,
                       [0, 1, 2, 3, 4]),
               54321);
    assert_eq!(amplify(&read_prog_from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")?,
                       [1, 0, 4, 3, 2]),
               65210);
    Ok(())
}

fn find_max_thrust(prog: &[i32]) -> i32 {
    let mut maxval = -1;
    let mut winner = None;
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                for l in 0..5 {
                    for m in 0..5 {
                        let phase_settings = [i, j, k, l, m];
                        if HashSet::<i32>::from_iter(phase_settings.iter().cloned()).len() != 5 {
                            continue;
                        }
                        let val = amplify(&prog, phase_settings);
                        if val > maxval {
                            maxval = val;
                            winner = Some(phase_settings);
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", winner);
    maxval
}

fn main() -> Result<(), Box<dyn Error>> {
    run_tests()?;
    let prog = read_prog(io::stdin().lock())?;
    println!("{}", find_max_thrust(&prog));
    Ok(())
}

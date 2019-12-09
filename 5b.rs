use std::io;
use std::io::prelude::*;
use std::error::Error;

fn interpret<'a>(mem: &mut [i32],
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

    fn decode_args_3(mem: &mut [i32], pos: usize) -> (i32, i32, i32) {
        let mut opcode = mem[pos] / 100;
        let immediate_1 = opcode % 10 == 1;
        opcode /= 10;
        let immediate_2 = opcode % 10 == 1;
        opcode /= 10;
        let immediate_3 = opcode % 10 == 1;
        (mem_read(mem, pos + 1, immediate_1),
         mem_read(mem, pos + 2, immediate_2),
         mem_read(mem, pos + 3, immediate_3))
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
    {
        let prog = read_prog_from("3,9,8,9,10,9,4,9,99,-1,8")?;
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 8), vec![1]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 7), vec![0]);
    }
    {
        let prog = read_prog_from("3,9,7,9,10,9,4,9,99,-1,8")?;
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 7), vec![1]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 8), vec![0]);
    }
    {
        let prog = read_prog_from("3,3,1108,-1,8,3,4,3,99")?;
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 8), vec![1]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 13), vec![0]);
    }
    {
        let prog = read_prog_from("3,3,1107,-1,8,3,4,3,99")?;
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 7), vec![1]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 8), vec![0]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 9), vec![0]);
    }
    {
        let prog = read_prog_from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")?;
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 0), vec![0]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 10), vec![1]);
    }
    {
        let prog = read_prog_from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")?;
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 0), vec![0]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 10), vec![1]);
    }
    {
        let prog = read_prog_from(concat!("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,",
                                          "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,",
                                          "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"))?;
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 7), vec![999]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 8), vec![1000]);
        assert_eq!(interpret(&mut prog.clone()[..], &mut || 9), vec![1001]);
    }

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    run_tests()?;

    let mut prog = read_prog(io::stdin().lock())?;
    println!("{:?}", interpret(&mut prog[..], &mut || 5));
    Ok(())
}

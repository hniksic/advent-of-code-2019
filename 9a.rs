#[derive(PartialEq, Debug)]
enum AddrMode {
    POSITION,
    IMMEDIATE,
    RELATIVE
}

fn interpret(mem: &mut Vec<i64>,
             read_input: &mut dyn FnMut() -> i64,
             write_output: &mut dyn FnMut(i64)) {
    fn read_direct(mem: &[i64], pos: usize) -> i64 {
        if pos >= mem.len() {
            0
        }
        else {
            mem[pos]
        }
    }
    fn mem_follow_mode(val: i64, mem: &[i64], relbase: usize, mode: AddrMode)
                    -> i64 {
        match mode {
            AddrMode::POSITION => read_direct(mem, val as usize),
            AddrMode::IMMEDIATE => val,
            AddrMode::RELATIVE => read_direct(mem, (relbase as i64 + val) as usize),
        }
    }
    fn mem_read(mem: &[i64], pos: usize, relbase: usize, mode: AddrMode) -> i64 {
        mem_follow_mode(read_direct(mem, pos), mem, relbase, mode)
    }

    fn decode_mode(digit: i64) -> AddrMode {
        match digit {
            0 => AddrMode::POSITION,
            1 => AddrMode::IMMEDIATE,
            2 => AddrMode::RELATIVE,
            other => panic!("invalid opcode {}", other),
        }
    }

    fn decode_next(mem: &mut [i64], pos: usize, relbase: usize, modes: i64) -> (i64, i64) {
        (mem_read(mem, pos, relbase, decode_mode(modes % 10)),
         modes / 10)
    }
    fn decode_operands_1(mem: &mut [i64], pos: usize, relbase: usize) -> (i64, i64) {
        let modes = mem[pos] / 100;
        decode_next(mem, pos + 1, relbase, modes)
    }
    fn decode_operands_2(mem: &mut [i64], pos: usize, relbase: usize) -> (i64, i64, i64) {
        let modes = mem[pos] / 100;
        let (val1, modes) = decode_next(mem, pos + 1, relbase, modes);
        let (val2, modes) = decode_next(mem, pos + 2, relbase, modes);
        (val1, val2, modes)
    }

    fn mem_write(mem: &mut Vec<i64>, relbase: usize,
                 raw_addr: i64, mode: AddrMode, val: i64) {
        assert_ne!(mode, AddrMode::IMMEDIATE);
        let pos = if let AddrMode::RELATIVE = mode {
            (raw_addr as isize + relbase as isize) as usize
        }
        else {raw_addr as usize};
        if pos >= mem.len() {
            mem.extend(std::iter::repeat(0).take(pos - mem.len() + 1));
        }
        mem[pos] = val;
    }

    let mut pos = 0usize;
    let mut relbase = 0usize;

    while mem[pos] != 99 {
        match mem[pos] % 100 {
            1 => {
                let (op1, op2, write_mode_raw) = decode_operands_2(mem, pos, relbase);
                let val = op1.checked_add(op2)
                    .expect(&format!("overflow {}+{}", op1, op2));
                mem_write(mem, relbase, mem[pos + 3], decode_mode(write_mode_raw), val);
                pos += 4;
            }
            2 => {
                let (op1, op2, write_mode_raw) = decode_operands_2(mem, pos, relbase);
                let val = op1.checked_mul(op2)
                    .expect(&format!("overflow {}*{}", op1, op2));
                mem_write(mem, relbase, mem[pos + 3], decode_mode(write_mode_raw), val);
                pos += 4;
            }
            3 => {
                let write_mode = decode_mode(mem[pos] / 100);
                let val = read_input();
                mem_write(mem, relbase, mem[pos + 1], write_mode, val);
                pos += 2;
            }
            4 => {
                let (val, modes) = decode_operands_1(mem, pos, relbase);
                assert_eq!(modes, 0);
                write_output(val);
                pos += 2;
            }
            5 => {
                let (op1, op2, modes) = decode_operands_2(mem, pos, relbase);
                assert_eq!(modes, 0);
                if op1 != 0 {
                    pos = op2 as usize;
                }
                else {
                    pos += 3;
                }
            }
            6 => {
                let (op1, op2, modes) = decode_operands_2(mem, pos, relbase);
                assert_eq!(modes, 0);
                if op1 == 0 {
                    pos = op2 as usize;
                }
                else {
                    pos += 3;
                }
            }
            7 => {
                let (op1, op2, write_mode_raw) = decode_operands_2(mem, pos, relbase);
                let val = if op1 < op2 {1} else {0};
                mem_write(mem, relbase, mem[pos + 3],
                          decode_mode(write_mode_raw), val);
                pos += 4;
            }
            8 => {
                let (op1, op2, write_mode_raw) = decode_operands_2(mem, pos, relbase);
                let val = if op1 == op2 {1} else {0};
                mem_write(mem, relbase, mem[pos + 3],
                          decode_mode(write_mode_raw), val);
                pos += 4;
            }
            9 => {
                let (val, modes) = decode_operands_1(mem, pos, relbase);
                assert_eq!(modes, 0);
                relbase = (relbase as isize + val as isize) as usize;
                pos += 2;
            }
            other => panic!("invalid instruction {}", other),
        };
    }
}


fn interpret_output(mem: &mut Vec<i64>, input: i64) -> Vec<i64> {
    let mut outvec = vec![];
    interpret(mem, &mut || input, &mut |val| outvec.push(val));
    outvec
}

fn run_tests() {
    assert_eq!(interpret_output(&mut vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], 0),
               vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    assert!(interpret_output(&mut vec![1102,34915192,34915192,7,4,7,99,0], 0)[0]
            >= 1_000_000_000_000_000);
    assert_eq!(interpret_output(&mut vec![104,1125899906842624,99], 0)[0],
               1125899906842624);
}

fn read_prog(input: &[u8]) -> Vec<i64> {
    let input = std::str::from_utf8(input).unwrap();
    let mut ret = vec![];
    for tok in input.split(",") {
        ret.push(tok.trim().parse().unwrap());
    }
    ret
}

fn main() {
    run_tests();

    let mut prog = read_prog(include_bytes!("9.input"));
    println!("{:?}", interpret_output(&mut prog, 1));
}

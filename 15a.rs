use std::sync::mpsc;
use std::collections::{VecDeque, HashSet};

fn interpret(mem: &mut Vec<i64>,
             read_input: &mut dyn FnMut() -> i64,
             write_output: &mut dyn FnMut(i64)) {
    #[derive(PartialEq, Debug)]
    enum AddrMode {
        POSITION,
        IMMEDIATE,
        RELATIVE
    }

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

fn reverse_step(step: i64) -> i64 {
    match step {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => unreachable!(),
    }
}

fn apply_step(pos: (i32, i32), step: i64) -> (i32, i32) {
    match step {
        1 => (pos.0, pos.1 - 1),
        2 => (pos.0, pos.1 + 1),
        3 => (pos.0 - 1, pos.1),
        4 => (pos.0 + 1, pos.1),
        _ => unreachable!(),
    }
}

fn find_fewest_steps(program: &[i64]) -> Option<usize> {
    let (in_tx, in_rx) = mpsc::channel();
    let (out_tx, out_rx) = mpsc::channel();

    let thr = {
        let program = program.to_vec();
        std::thread::spawn(
            move ||
                interpret(&mut program.to_vec(),
                          &mut || in_rx.recv().unwrap(),
                          &mut |val| out_tx.send(val).unwrap())
        )
    };

    let mut result = None;
    let mut visited = HashSet::new();

    let mut todo = VecDeque::new();
    todo.push_back(vec![]);
    'outer:
    while let Some(path) = todo.pop_front() {
        let mut current_pos: (i32, i32) = (0, 0);
        for &old_step in &path {
            in_tx.send(old_step).unwrap();
            current_pos = apply_step(current_pos, old_step);
        }
        for _ in 0..path.len() {
            assert_eq!(out_rx.recv().unwrap(), 1);
        }

        for &new_step in &[1, 2, 3, 4] {
            if visited.contains(&apply_step(current_pos, new_step)) {
                continue;
            }
            visited.insert(apply_step(current_pos, new_step));
            in_tx.send(new_step).unwrap();
            match out_rx.recv().unwrap() {
                0 => continue,
                1 => (),
                2 => {
                    result = Some(path.len() + 1);
                    break 'outer;
                }
                _ => unreachable!(),
            }
            in_tx.send(reverse_step(new_step)).unwrap();
            assert_eq!(out_rx.recv().unwrap(), 1);
            let mut new_path = path.clone();
            new_path.push(new_step);
            todo.push_back(new_path);
        }
        for &old_step in path.iter().rev() {
            in_tx.send(reverse_step(old_step)).unwrap();
        }
        for _ in 0..path.len() {
            assert_eq!(out_rx.recv().unwrap(), 1);
        }
    }

    let old_hook = std::panic::take_hook();
    // prevent panic output when channel recv() panics
    std::panic::set_hook(Box::new(|_| ()));
    drop(in_tx);
    drop(out_rx);
    let _ = thr.join();
    std::panic::set_hook(old_hook);
    result
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
    let prog = read_prog(include_bytes!("15.input"));
    println!("{:?}", find_fewest_steps(&prog));
}

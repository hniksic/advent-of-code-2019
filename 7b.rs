use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::sync::mpsc;
use std::thread;

fn interpret(mem: &mut [i32],
             read_input: &mut dyn FnMut() -> i32,
             write_output: &mut dyn FnMut(i32)) {
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
                write_output(val);
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
}

fn amplify(program: &[i32], phase_settings: [i32; 5]) -> i32 {
    let (first_tx, mut cur_rx) = mpsc::channel();
    let mut threads = vec![];

    for (i, &phase) in phase_settings.iter().enumerate() {
        let mut running = program.to_vec();
        let (tx, rx) = mpsc::channel();
        let mut get_input = {
            let mut first = true;
            move || {
                if first {
                    first = false;
                    phase
                } else {
                    cur_rx.recv().expect("failed to receive input")
                }
            }
        };
        let mut set_output = move |val| {
            tx.send(val).expect("failed to send output");
        };
        threads.push(
            thread::Builder::new()
                .name(format!("amp{}", i)).spawn(
                    move || interpret(&mut running,
                                      &mut get_input, &mut set_output)
                ).unwrap());
        cur_rx = rx;
    }

    let (thrust_tx, thrust_rx) = mpsc::channel();
    threads.push(thread::spawn(move || {
        first_tx.send(0).expect("failed to send seed value");
        while let Some(val) = cur_rx.recv().ok() {
            thrust_tx.send(val).expect("failed to send output");
            if let Err(_) = first_tx.send(val) {
                break
            }
        }
    }));
    for t in threads {
        t.join().expect("spread the panic");
    }
    let mut thrust = 0;
    while let Some(val) = thrust_rx.try_recv().ok() {
        thrust = val;
    }
    thrust
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
    assert_eq!(amplify(&read_prog_from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")?,
                       [9,8,7,6,5]),
               139629729);
    assert_eq!(amplify(&read_prog_from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")?,
                       [9,7,8,5,6]),
               18216);

        Ok(())
}

fn find_max_thrust(prog: &[i32]) -> i32 {
    let mut maxval = -1;
    let mut winner = None;
    for i in 5..10 {
        for j in 5..10 {
            if j == i {
                continue;
            }
            for k in 5..10 {
                if k == i || k == j {
                    continue;
                }
                for l in 5..10 {
                    if l == i || l == j || l == k {
                        continue;
                    }
                    for m in 5..10 {
                        if m == i || m == j || m == k || m == l {
                            continue;
                        }
                        let phase_settings = [i, j, k, l, m];
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

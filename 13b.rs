use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
//use std::iter::FromIterator;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    EMPTY, WALL, BLOCK, PADDLE, BALL
}
use Tile::*;

impl Tile {
    fn decode(val: i64) -> Tile {
        match val {
            0 => EMPTY,
            1 => WALL,
            2 => BLOCK,
            3 => PADDLE,
            4 => BALL,
            other => panic!("invalid tile {}", other),
        }
    }
}

fn all_blocks_broken(screen: &HashMap<(i64, i64), Tile>) -> bool {
    screen.values().all(|&v| v != Tile::BLOCK)
}

fn locate(screen: &HashMap<(i64, i64), Tile>, tile: Tile) -> (i64, i64) {
    screen.iter()
        .filter_map(|(&k, &v)| if { v == tile } { Some(k) } else { None })
        .next().unwrap()
}

// fn count_tiles(screen: &HashMap<(i64, i64), Tile>,
//                tile_type: Tile) -> usize {
//     screen.values().filter(|&&v| v == tile_type).count()
// }

// fn display(screen: &HashMap<(i64, i64), Tile>, score: i64) {
//     print!("\x1b[2J\x1b[H");
//     println!("Score: {}", score);
//     println!("Blocks: {}", count_tiles(screen, BLOCK));
//     let max_x = screen.keys().map(|&(x, _y)| x).max().unwrap() as usize;
//     let max_y = screen.keys().map(|&(_x, y)| y).max().unwrap() as usize;

//     let mut img: Vec<Vec<char>> = std::iter::repeat_with(
//         || std::iter::repeat(' ').take(max_x + 1).collect()
//     ).take(max_y + 1).collect();
//     for (&(x, y), tile) in screen.iter() {
//         assert!(x >= 0 && y >= 0);
//         img[y as usize][x as usize] = match tile {
//             EMPTY => ' ',
//             WALL => 'X',
//             BLOCK => '#',
//             PADDLE => '-',
//             BALL => '*',
//         }
//     }
//     for row in img {
//         println!("{}", String::from_iter(row));
//     }
//     std::thread::sleep(std::time::Duration::from_micros(500));
// }

fn beat_pong(program: &[i64]) -> i64 {
    let screen = Rc::new(RefCell::new(HashMap::new()));
    let score1 = Rc::new(Cell::new(None));
    {
        let mut score = 0;
        let (mut x, mut y) = (None, None);
        let mut output = {
            let screen = Rc::clone(&screen);
            let score1 = Rc::clone(&score1);
            move |val| {
                match (x, y) {
                (None, None) => {
                    x = Some(val);
                    y = None;
                }
                (Some(_), None) => {
                    y = Some(val);
                }
                (Some(xval), Some(yval)) => {
                    if (xval, yval) != (-1, 0) {
                        screen.borrow_mut().insert((xval, yval), Tile::decode(val));
                    }
                    else {
                        score = val;
                        if all_blocks_broken(&*screen.borrow()) {
                            score1.set(Some(score));
                        }
                    }
                    x = None;
                    y = None;
                    //display(&*screen.borrow(), score);
                }
                _ => unreachable!(),
                }
            }
        };

        let mut input = {
            let screen = Rc::clone(&screen);
            move || {
                let screen = &*screen.borrow();
                let paddle_pos = locate(screen, Tile::PADDLE).0;
                let ball_pos = locate(screen, Tile::BALL).0;
                if ball_pos < paddle_pos { -1 }
                else if ball_pos > paddle_pos { 1 }
                else { 0 }
            }
        };

        interpret(&mut program.to_vec(),
                  &mut input,
                  &mut output);
    }
    score1.get().unwrap()
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
    let mut prog = read_prog(include_bytes!("13.input"));
    prog[0] = 2;
    println!("{}", beat_pong(&prog));
}

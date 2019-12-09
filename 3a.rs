use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction { U, D, R, L }
use Direction::*;

struct Move(Direction, usize);

fn trace(path: &[Move]) -> Vec<(isize, isize)> {
    let mut out = vec![];
    let (mut x, mut y) = (0, 0);
    for &Move(dir, size) in path {
        let (horizontal, inc);
        match dir {
            U => { horizontal = false; inc = -1; }
            D => { horizontal = false; inc = 1; }
            L => { horizontal = true; inc = -1; }
            R => { horizontal = true; inc = 1; }
        }
        for _ in 0..size {
            *if horizontal { &mut x } else { &mut y } += inc;
            out.push((x, y));
        }
    }
    out
}

fn intersections(p1: &[Move], p2: &[Move]) -> Vec<(isize, isize)> {
    let points1: HashSet<_> = trace(p1).into_iter().collect();
    let points2: HashSet<_> = trace(p2).into_iter().collect();
    points1.intersection(&points2).cloned().collect()
}

fn closest_intersection(p1: &[Move], p2: &[Move]) -> usize {
    intersections(p1, p2).into_iter()
        .map(|p| (p.0.abs() + p.1.abs()) as usize)
        .min()
        .unwrap_or(0)
}

fn parse(s: &str) -> Vec<Move> {
    s.split(",").map(|tok| Move(
        match tok.chars().next().unwrap() {
            'U' => U, 'D' => D, 'R' => R, 'L' => L,
            other => panic!("got {}", other)
        },
        tok[1..].parse().expect("number expected")
    )).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(closest_intersection(&parse("R8,U5,L5,D3"), &parse("U7,R6,D4,L4")), 6);
    assert_eq!(closest_intersection(&parse("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                                    &parse("U62,R66,U55,R34,D71,R55,D58,R83")), 159); 
    assert_eq!(closest_intersection(&parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                                    &parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")), 135);

    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    let l1 = stdin_lines.next().unwrap_or_else(|| Ok("".to_string()))?;
    let l2 = stdin_lines.next().unwrap_or_else(|| Ok("".to_string()))?;
    println!("{}", closest_intersection(&parse(&l1), &parse(&l2)));

    Ok(())
}

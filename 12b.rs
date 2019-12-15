use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Copy, Clone)]
struct Moon {
    pos: Coord,
    vel: Coord,
}

fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            match moons[i].pos.x.cmp(&moons[j].pos.x) {
                Ordering::Less => moons[i].vel.x += 1,
                Ordering::Greater => moons[i].vel.x -= 1,
                Ordering::Equal => (),
            }
            match moons[i].pos.y.cmp(&moons[j].pos.y) {
                Ordering::Less => moons[i].vel.y += 1,
                Ordering::Greater => moons[i].vel.y -= 1,
                Ordering::Equal => (),
            }
            match moons[i].pos.z.cmp(&moons[j].pos.z) {
                Ordering::Less => moons[i].vel.z += 1,
                Ordering::Greater => moons[i].vel.z -= 1,
                Ordering::Equal => (),
            }
        }
    }
}

fn apply_velocity(moons: &mut [Moon]) {
    for m in moons {
        m.pos.x += m.vel.x;
        m.pos.y += m.vel.y;
        m.pos.z += m.vel.z;
    }
}

#[derive(Copy, Clone)]
enum Axis { X, Y, Z }

fn get_state(moons: &[Moon], axis: Axis) -> Vec<(i32, i32)> {
    let get_axis: &dyn Fn(&Moon) -> (i32, i32) = match axis {
        Axis::X => &|m| (m.pos.x, m.vel.x),
        Axis::Y => &|m| (m.pos.y, m.vel.y),
        Axis::Z => &|m| (m.pos.z, m.vel.z),
    };
    moons.iter().map(get_axis).collect()
}

fn find_period(moons: &[Moon], axis: Axis) -> usize {
    let mut moons = moons.to_vec();
    let mut nsteps = 0usize;
    let orig_state = get_state(&moons, axis);
    loop {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
        nsteps += 1;
        let current_state = get_state(&moons, axis);
        if current_state == orig_state {
            return nsteps;
        }
    }
}

fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn find_repeating(moons: &[Moon]) -> usize {
    let x_period = find_period(moons, Axis::X);
    let y_period = find_period(moons, Axis::Y);
    let z_period = find_period(moons, Axis::Z);
    lcm(x_period, lcm(y_period, z_period))
}

fn match_const<'a>(s: &'a str, prefix: &str) -> &'a str {
    assert!(s.starts_with(prefix));
    &s[prefix.len()..]
}

fn match_ws<'a>(s: &'a str) -> &'a str {
    for (ind, c) in s.char_indices() {
        if !c.is_ascii_whitespace() {
            return &s[ind..];
        }
    }
    return "";
}

fn match_num(s: &str) -> (i32, &str) {
    let mut end_num = 0;
    for (ind, c) in s.char_indices() {
        end_num = ind;
        if ind == 0 && c == '+' || c == '-' {
            continue;
        }
        if !c.is_ascii_digit() {
            break;
        }
    }
    (s[..end_num].parse().unwrap(), &s[end_num..])
}

fn read_coord(s: &str) -> (Coord, &str) {
    let s = match_const(s, "<x=");
    let (x, s) = match_num(s);
    let s = match_const(s, ", y=");
    let (y, s) = match_num(s);
    let s = match_const(s, ", z=");
    let (z, s) = match_num(s);
    let s = match_const(s, ">");
    let s = match_ws(s);
    (Coord { x, y, z }, s)
}

fn read_moons(mut s: &str) -> Vec<Moon> {
    let mut moons = vec![];
    let zero = Coord { x: 0, y: 0, z: 0 };
    while s.len() != 0 {
        let (pos, rest) = read_coord(s);
        moons.push(Moon { pos, vel: zero });
        s = rest;
    }
    moons
}

fn run_tests() {
    let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
    let moons = read_moons(input);
    assert_eq!(find_repeating(&moons), 2772);
}

fn main() {
    run_tests();

    let input = include_str!("12.input");
    let moons = read_moons(input);
    print!("{}", find_repeating(&moons));
}

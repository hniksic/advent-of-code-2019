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

impl Moon {
    fn pot(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }
    fn kin(&self) -> i32 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }
    fn total(&self) -> i32 {
        self.pot() * self.kin()
    }
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

fn simulate(moons: &mut [Moon], nsteps: usize) -> i32 {
    for _ in 0..nsteps {
        apply_gravity(moons);
        apply_velocity(moons);
    }
    moons.iter().map(Moon::total).sum()
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

fn main() {
    let input = include_str!("12.input");
    let mut moons = read_moons(input);
    print!("{}", simulate(&mut moons, 1000));
}

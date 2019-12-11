use std::f64::consts::PI;

fn gcd(x: isize, y: isize) -> isize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn find_target(asteroids: &[(isize, isize)],
               (x1, y1): (isize, isize),
               (x2, y2): (isize, isize)) -> (isize, isize) {
    let (step_x, step_y);
    if x1 == x2 {
        step_x = 0;
        step_y = if y2 > y1 { 1 } else { -1 };
    }
    else if y1 == y2 {
        step_x = if x2 > x1 { 1 } else { -1 };
        step_y = 0;
    }
    else {
        let gcd = gcd((x1 - x2).abs(), (y1 - y2).abs());
        step_x = (x2 - x1) / gcd;
        step_y = (y2 - y1) / gcd;
    }
    let (mut x, mut y) = (x1 + step_x, y1 + step_y);
    while (x, y) != (x2, y2) {
        if let Some(_) = asteroids.iter().find(|&&pos| pos == (x, y)) {
            return (x, y);
        }
        x += step_x;
        y += step_y;
    }
    (x2, y2)
}

fn find_victims(asteroids: &[(isize, isize)],
                laser_pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut victims = vec![];
    for &(x, y) in asteroids.iter() {
        if (x, y) == laser_pos {
            continue;
        }
        let target = find_target(asteroids, laser_pos, (x, y));
        if victims.iter().find(|&&v| v == target).is_some() {
            continue;
        }
        victims.push(target);
    }
    victims
}

#[derive(PartialOrd, PartialEq)]
struct SortableFloat(f64);

impl Ord for SortableFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Eq for SortableFloat {
}

fn angle(laser: (isize, isize), victim: (isize, isize)) -> SortableFloat {
    // negate the y because larger y values are *down*, not up
    let mut angle = (-(victim.1 - laser.1) as f64).atan2((victim.0 - laser.0) as f64);
    // now: [-PI, PI] range
    if angle < 0. {
        angle += 2.*PI;
    } // now: [0, 2*PI] range
    angle = 2.*PI-angle;       // now: [0, 2*PI] clockwise
    angle = angle - 3./2.*PI;
    if angle < 0. {
        angle += 2.*PI;
    }
    SortableFloat(angle)
}

fn vaporize_many(asteroids: &[(isize, isize)],
                 laser_pos: (isize, isize),
                 n_shots: usize) -> (isize, isize) {
    let mut victims = find_victims(asteroids, laser_pos);
    victims.sort_by_key(|&v| angle(laser_pos, v));
    let pos0 = victims.iter().position(|&v| angle(laser_pos, v).0 >= 0.0).unwrap();
    return victims[pos0 + n_shots - 1];
}

fn read_img(data: &str) -> Vec<(isize, isize)> {
    data.split('\n')
        .enumerate()
        .flat_map(|(y, row)|
                  row.chars().enumerate()
                  .filter_map(move |(x, c)| match c {
                      '.' => None,
                      '#' => Some((x as isize, y as isize)),
                      other => panic!("invalid char {}", other),
                  }))
        .collect()
}

fn run_tests() {
}

fn main() {
    run_tests();
    let asteroids = read_img(std::str::from_utf8(include_bytes!("10.input")).unwrap());
    let laser_pos = (31, 20);  // from 10a
    let (x, y) = vaporize_many(&asteroids, laser_pos, 200);
    println!("{}", x * 100 + y);
}

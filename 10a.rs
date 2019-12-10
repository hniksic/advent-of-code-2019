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

fn has_occlusions(asteroids: &[(isize, isize)],
                  (x1, y1): (isize, isize),
                  (x2, y2): (isize, isize)) -> bool {
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
        if asteroids.iter().find(|&&pos| pos == (x, y)).is_some() {
            return true;
        }
        x += step_x;
        y += step_y;
    }
    false
}

fn find_best_asteroid(asteroids: &[(isize, isize)]) -> (isize, (isize, isize)) {
    let mut totalcnt = 0;
    let mut winner = None;
    for &(x, y) in asteroids.iter() {
        let mut cnt = 0;
        for &(x1, y1) in asteroids.iter() {
            if (x, y) == (x1, y1) {
                continue;
            }
            if has_occlusions(asteroids, (x, y), (x1, y1)) {
                continue;
            }
            cnt += 1;
        }
        if cnt > totalcnt {
            totalcnt = cnt;
            winner = Some((x, y));
        }
    }
    (totalcnt, winner.unwrap())
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
    assert_eq!(find_best_asteroid(
        &read_img(".#..#\n.....\n#####\n....#\n...##\n")),
               (8, (3, 4)));
    assert_eq!(find_best_asteroid(
        &read_img(concat!("......#.#.\n",
                          "#..#.#....\n",
                          "..#######.\n",
                          ".#.#.###..\n",
                          ".#..#.....\n",
                          "..#....#.#\n",
                          "#..#....#.\n",
                          ".##.#..###\n",
                          "##...#..#.\n",
                          ".#....####\n",))),
               (33, (5, 8)));
    assert_eq!(find_best_asteroid(
        &read_img(concat!(
            ".#..##.###...#######\n",
            "##.############..##.\n",
            ".#.######.########.#\n",
            ".###.#######.####.#.\n",
            "#####.##.#.##.###.##\n",
            "..#####..#.#########\n",
            "####################\n",
            "#.####....###.#.#.##\n",
            "##.#################\n",
            "#####.##.###..####..\n",
            "..######..##.#######\n",
            "####.##.####...##..#\n",
            ".#####..#.######.###\n",
            "##...#.##########...\n",
            "#.##########.#######\n",
            ".####.#.###.###.#.##\n",
            "....##.##.###..#####\n",
            ".#.#.###########.###\n",
            "#.#.#.#####.####.###\n",
            "###.##.####.##.#..##\n",))),
               (210, (11, 13)));
}

fn main() {
    run_tests();
    let img = read_img(std::str::from_utf8(include_bytes!("10.input")).unwrap());
    println!("{:?}", find_best_asteroid(&img));
}

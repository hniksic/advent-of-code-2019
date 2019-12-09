use std::error::Error;

fn check_two_adjacent_eq(pwd: &[u8; 6]) -> bool {
    let mut current_digit = None;
    let mut run_size = 0;
    for i in 0..pwd.len() {
        if Some(pwd[i]) == current_digit {
            run_size += 1;
            continue;
        }
        if run_size == 2 {
            return true;
        }
        current_digit = Some(pwd[i]);
        run_size = 1;
    }
    return run_size == 2;
}

fn check_non_decreasing(pwd: &[u8; 6]) -> bool {
    (0..pwd.len()-1).all(|i| pwd[i] <= pwd[i + 1])
}

fn check(pwd: &[u8; 6]) -> bool {
    check_two_adjacent_eq(pwd) && check_non_decreasing(pwd)
}

fn bump(pwd: &mut [u8; 6]) {
    for digit in pwd.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            return;
        }
        *digit = 0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    assert!(check(&[1,1,2,2,3,3]));
    assert!(!check(&[1,2,3,4,4,4]));
    assert!(check(&[1,1,1,1,2,2]));

    let first = [2,4,0,9,2,0];
    let last = [7,8,9,8,5,7];

    let mut cnt = 0;
    let mut pwd = first.clone();
    while pwd != last {
        if check(&pwd) {
            cnt += 1;
        }
        bump(&mut pwd);
    }
    println!("{}", cnt);

    Ok(())
}

use std::io;
use std::io::prelude::*;

fn calc_fuel(mass: u32) -> u32 {
    let tmp = mass / 3;
    if tmp > 2 { tmp - 2 } else { 0 }
}

fn calc_fuel_rec(mut mass: u32) -> u32 {
    let mut total = 0;
    while mass != 0 {
        let fuel = calc_fuel(mass);
        total += fuel;
        mass = fuel;
    }
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(calc_fuel_rec(14), 2);
    assert_eq!(calc_fuel_rec(1969), 966);
    assert_eq!(calc_fuel_rec(100756), 50346);

    let stdin = io::stdin();
    let mut total_fuel = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        total_fuel += calc_fuel_rec(line.parse()?);
    }
    println!("{}", total_fuel);
    Ok(())
}

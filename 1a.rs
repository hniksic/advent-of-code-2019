use std::io;
use std::io::prelude::*;
use std::error::Error;

fn fuel(mass: u32) -> u32 {
    return mass / 3 - 2;
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);

    let stdin = io::stdin();
    let mut total_fuel = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let mass: u32 = line.parse()?;
        total_fuel += fuel(mass);
    }
    println!("{}", total_fuel);
    Ok(())
}

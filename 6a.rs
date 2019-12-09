use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug)]
struct Body {
    name: String,
    orbiting: String
}

#[derive(Debug)]
struct Map {
    bodies: Vec<Body>,
    by_name: HashMap<String, usize>,
}

impl Map {
    fn read(input: impl BufRead) -> io::Result<Map> {
        let mut bodies = vec![];
        let mut by_name = HashMap::new();
        for line in input.lines() {
            let line = line?;
            let mut tokiter = line.split(")");
            let orbitee = tokiter.next().unwrap();
            let orbiter = tokiter.next().unwrap().trim();
            by_name.insert(orbiter.to_owned(), bodies.len());
            bodies.push(Body {
                name: orbiter.to_owned(),
                orbiting: orbitee.to_owned()
            });
        }
        Ok(Map { bodies, by_name })
    }

    fn find_body(&self, name: &str) -> Option<&Body> {
        self.by_name.get(name).map(|&ind| &self.bodies[ind])
    }

    fn count_orbits_of(&self, body: &Body) -> usize {
        if let Some(orbitee) = self.find_body(&body.orbiting) {
            1 + self.count_orbits_of(orbitee)
        }
        else {
            assert!(body.orbiting == "COM");
            1
        }
    }

    fn count_all_orbits(&self) -> usize {
        self.bodies.iter().map(|b| self.count_orbits_of(b)).sum()
    }
}

fn run_tests() {
    let map = Map::read(&b"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"[..]).unwrap();
    assert_eq!(map.count_all_orbits(), 42);
}

fn main() -> Result<(), Box<dyn Error>> {
    run_tests();
    let map = Map::read(io::stdin().lock())?;
    println!("{:?}", map.count_all_orbits());
    Ok(())
}

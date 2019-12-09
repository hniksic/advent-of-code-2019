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

    fn count_distance(&self, name1: &str, name2: &str) -> usize {
        let mut distances = HashMap::new();
        {
            let mut dist1 = 0;
            let mut name = name1;
            while let Some(b) = self.find_body(name) {
                distances.insert(&b.orbiting, dist1);
                name = &b.orbiting;
                dist1 += 1usize;
            }
        }

        {
            let mut dist2 = 0;
            let mut name = name2;
            while let Some(b) = self.find_body(name) {
                if let Some(dist1) = distances.get(&b.orbiting) {
                    return dist1 + dist2;
                }
                name = &b.orbiting;
                dist2 += 1;
            }
        }
        unreachable!();
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
K)L
K)YOU
I)SAN"[..]).unwrap();
    assert_eq!(map.count_distance("YOU", "SAN"), 4);
}

fn main() -> Result<(), Box<dyn Error>> {
    run_tests();
    let map = Map::read(io::stdin().lock())?;
    println!("{:?}", map.count_distance("YOU", "SAN"));
    Ok(())
}

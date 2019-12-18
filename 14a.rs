use std::collections::HashMap;

fn match_ws<'a>(s: &'a str) -> &'a str {
    for (ind, c) in s.char_indices() {
        if !c.is_ascii_whitespace() {
            return &s[ind..];
        }
    }
    return "";
}

fn match_num<'a, T>(s: &str) -> (T, &str)
    where T: std::str::FromStr,
          <T as std::str::FromStr>::Err: std::fmt::Debug
{
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

fn match_word(s: &str) -> (&str, &str) {
    let mut end = None;
    for (ind, c) in s.char_indices() {
        if c.is_ascii_whitespace() {
            end = Some(ind);
            break;
        }
    }
    let end = end.unwrap_or(s.len());
    (&s[..end], &s[end..])
}

fn match_ident(s: &str) -> (&str, &str) {
    let mut end = None;
    for (ind, c) in s.char_indices() {
        if !c.is_ascii_alphanumeric() {
            end = Some(ind);
            break;
        }
    }
    let end = end.unwrap_or(s.len());
    assert!(end != 0);
    (&s[..end], &s[end..])
}

fn read_reaction(mut input: &str) -> Reaction {
    let mut src = vec![];
    while input.len() != 0 {
        let s = input;
        let (qty, s) = match_num(s);
        let s = match_ws(s);
        let (name, s) = match_ident(s);
        let s = match_ws(s);
        let (tok, s) = match_word(s);
        src.push(Material { qty, name: name.to_string() });
        if tok == "=>" {
            input = s;
            break;
        }
        assert_eq!(tok, ",");
        let s = match_ws(s);
        input = s;
    }
    let s = input;
    let s = match_ws(s);
    let (qty, s) = match_num(s);
    let s = match_ws(s);
    let (name, s) = match_ident(s);
    let s = match_ws(s);
    assert_eq!(s, "");
    Reaction {
        input: src,
        output: Material { qty, name: name.to_string() },
    }
}

fn read_reaction_vec(s: &str) -> Vec<Reaction> {
    s.lines().map(read_reaction).collect()
}

#[derive(Debug)]
struct Material {
    qty: u32,
    name: String,
}

#[derive(Debug)]
struct Reaction {
    input: Vec<Material>,
    output: Material,
}

fn ceil_div(x: u32, y: u32) -> u32 {
    x / y + if x % y != 0 { 1 } else { 0 }
}

fn calc_ore(reactions: &[Reaction], want_name: &str, want_qty: u32,
            surplus: &mut HashMap<String, u32>) -> u32 {
    if want_name == "ORE" {
        return want_qty;
    }
    let r = reactions.iter().find(|r| r.output.name == want_name).unwrap();
    let nreactions = ceil_div(want_qty, r.output.qty);
    let mut ore_input = 0;
    for _ in 0..nreactions {
        for m in &r.input {
            let mut qty = m.qty;
            if let Some(&surplus_qty) = surplus.get(&m.name) {
                if surplus_qty >= qty {
                    surplus.insert(m.name.clone(), surplus_qty - qty);
                    continue;
                } else {
                    qty -= surplus_qty;
                    surplus.remove(&m.name);
                }
            }
            ore_input += calc_ore(reactions, &m.name, qty, surplus);
        }
    }
    let created_qty = r.output.qty * nreactions;
    assert!(created_qty >= want_qty);
    *surplus.entry(want_name.to_string()).or_insert(0)
        += created_qty - want_qty;
    ore_input
}

fn calc_fuel(reactions: &[Reaction]) -> u32 {
    calc_ore(reactions, "FUEL", 1, &mut HashMap::new())
}

fn run_tests() {
    assert_eq!(calc_fuel(&read_reaction_vec("10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL")), 31);

    assert_eq!(calc_fuel(&read_reaction_vec("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL")), 165);

assert_eq!(calc_fuel(&read_reaction_vec("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT")), 13312);
}

fn main() {
    run_tests();

    let input = include_str!("14.input");
    println!("{:?}", calc_fuel(&read_reaction_vec(input)));
}

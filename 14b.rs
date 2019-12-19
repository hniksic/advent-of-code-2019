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
    qty: u64,
    name: String,
}

#[derive(Debug)]
struct Reaction {
    input: Vec<Material>,
    output: Material,
}

fn ceil_div(x: u64, y: u64) -> u64 {
    x / y + if x % y != 0 { 1 } else { 0 }
}

fn calc_ore(reactions: &[Reaction], want_name: &str, want_qty: u64,
            surplus: &mut HashMap<String, u64>) -> u64 {
    if want_name == "ORE" {
        return want_qty;
    }
    let r = reactions.iter().find(|r| r.output.name == want_name).unwrap();
    let nreactions = ceil_div(want_qty, r.output.qty);
    let mut ore_input = 0;
    for m in &r.input {
        let mut qty = m.qty * nreactions;
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
    let created_qty = r.output.qty * nreactions;
    assert!(created_qty >= want_qty);
    *surplus.entry(want_name.to_string()).or_insert(0)
        += created_qty - want_qty;
    ore_input
}

fn calc_total_fuel(reactions: &[Reaction], stored_ore: u64) -> u64 {
    let mut low_fuel: u64 = 1;
    let mut high_fuel = None;

    loop {
        let test_fuel = if let Some(high_fuel) = high_fuel {
            low_fuel + (high_fuel - low_fuel) / 2
        } else {
            low_fuel.checked_mul(2).unwrap()
        };

        let ore = calc_ore(reactions, "FUEL", test_fuel, &mut HashMap::new());
        if ore < stored_ore {
            if Some(low_fuel) == high_fuel || Some(low_fuel + 1) == high_fuel {
                break test_fuel;
            }
            low_fuel = test_fuel;
        } else if ore > stored_ore {
            high_fuel = Some(test_fuel);
        } else {
            break test_fuel;
        }
    }
}

fn run_tests() {
    let test = |input, num| {
        assert_eq!(calc_total_fuel(&read_reaction_vec(input),
                                  1_000_000_000_000), num);
    };

    test("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT", 82892753);

    test("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF", 5586022);

    test("171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX", 460664);
}

fn main() {
    run_tests();

    let input = include_str!("14.input");
    println!("{:?}", calc_total_fuel(&read_reaction_vec(input), 1_000_000_000_000));
}

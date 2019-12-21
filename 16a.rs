fn pattern(pos: usize) -> impl Iterator<Item = i8> {
    let base = &[0, 1, 0, -1];
    base.iter()
        .cloned()
        .flat_map(move |n| std::iter::repeat(n).take(pos))
        .cycle()
        .skip(1)
}

fn fft(input: &[u8], nphases: usize) -> Vec<u8> {
    let mut lst = input.to_vec();
    for _ in 0..nphases {
        let mut new = lst.to_vec();
        for (i, newelem) in new.iter_mut().enumerate() {
            let newval = lst.iter().zip(pattern(i + 1))
                .map(|(&elem, patval)| (elem as i32) * (patval as i32))
                .sum::<i32>()
                .abs() % 10;
            *newelem = newval as u8;
        }
        lst = new;
    }
    lst
}

fn run_tests() {
    assert_eq!(fft(&[1,2,3,4,5,6,7,8], 4), &[0,1,0,2,9,4,9,8]);
    assert_eq!(&fft(&[8,0,8,7,1,2,2,4,5,8,5,9,1,4,5,4,6,6,1,9,0,8,3,2,1,8,6,4,5,5,9,5], 100)[0..8],
               &[2,4,1,7,6,1,7,6]);
    assert_eq!(&fft(&[1,9,6,1,7,8,0,4,2,0,7,2,0,2,2,0,9,1,4,4,9,1,6,0,4,4,1,8,9,9,1,7], 100)[0..8],
               &[7,3,7,4,5,4,1,8]);
    assert_eq!(&fft(&[6,9,3,1,7,1,6,3,4,9,2,9,4,8,6,0,6,3,3,5,9,9,5,9,2,4,3,1,9,8,7,3], 100)[0..8],
               &[5,2,4,3,2,1,3,3]);
}

fn main() {
    run_tests();
    let input: Vec<u8> = include_str!("16.input")
        .trim()
        .chars()
        .map(|c| c as u8 - b'0')
        .collect();
    println!("{:?}", &fft(&input, 100)[0..8]);
}

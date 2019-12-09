fn analyze(buf: &[u8], width: usize, height: usize) -> usize {
    let mut counts = vec![];
    for layer_data in buf.chunks(width * height) {
        let (mut cnt0, mut cnt1, mut cnt2) = (0usize, 0usize, 0usize);
        for val in layer_data {
            match val - b'0' {
                0 => cnt0 += 1,
                1 => cnt1 += 1,
                2 => cnt2 += 1,
                _ => (),
            }
        }
        counts.push((cnt0, cnt1, cnt2));
    }
    counts.iter()
        .min_by_key(|(cnt0, _cnt1, _cnt2)| cnt0)
        .map(|(_cnt0, cnt1, cnt2)| cnt1 * cnt2)
        .unwrap()
}

fn trim(data: &[u8]) -> &[u8] {
    let mut end = data.len();
    while data[end - 1] == b'\n' {
        end -= 1;
    }
    &data[..end]
}

fn main() {
    let data = include_bytes!("8.input");
    println!("{}", analyze(trim(&data[..]), 25, 6));
}

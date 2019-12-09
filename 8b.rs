fn decode(buf: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut image: Option<Vec<u8>> = None;
    assert_eq!(buf.len() % (width * height), 0);
    for layer_data in buf.chunks(width * height).rev() {
        match image.as_mut() {
            None => image = Some(layer_data.iter()
                                 .map(|&raw_val| raw_val - b'0').collect()),
            Some(image) => {
                for (pos, &raw_val) in layer_data.iter().enumerate() {
                    let val = raw_val - b'0';
                    if val != 2 {
                        image[pos] = val;
                    }
                }
            }
        }
    }
    image.unwrap()
}

fn trim(data: &[u8]) -> &[u8] {
    let mut end = data.len();
    while data[end - 1] == b'\n' {
        end -= 1;
    }
    &data[..end]
}

fn display(image: &[u8], width: usize) {
    for row in image.chunks(width) {
        let row_str: String = row.iter().map(|p| match p {
            0 => '\u{2588}', 1 => ' ', 2 => ' ',
            other => panic!("invalid image data: {}", other),
        }).collect();
        println!("{}", row_str);
    }
}

fn main() {
    let data = trim(include_bytes!("8.input"));
    display(&decode(data, 25, 6), 25);
}

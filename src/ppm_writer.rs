use std::fs::File;
use std::io::Write;

pub fn write(filename: &str, width: usize, height: usize, data: &[u8]) {
    assert_eq!(data.len(), 3 * width * height);
    let mut file = File::create(filename).unwrap();
    let header = format!("P6 {} {} 255\n", width, height);
    file.write(header.as_bytes()).expect("");
    file.write(&data).expect("");
}

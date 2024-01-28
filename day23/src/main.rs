extern crate ndarray;


fn build_map(data: &[u8]) -> ndarray::Array2::<u8> {
    let m = 
        data.split(|b| *b == b'\n')
            .filter(|l| l.len() != 0)
            .count();
    let n = 
        data.split(|b| *b == b'\n')
            .next()
            .unwrap()
            .len();
    let mut map = ndarray::Array2::<u8>::zeros((m, n));
    for (i, line) in data.split(|b| *b == b'\n').enumerate() {
        for (j, c) in line.iter().enumerate() {
            map[[i, j]] = match c {
                b'.' => 0,
                b'#' => 1,
                _ => panic!("Invalid input"),
            }
        }
    }
    map
}


fn main() {
    println!("Hello, world!");
    let data = include_bytes!("../input.txt");
    let mut map = build_map(data);
    let (mut m, mut n) = map.dim();
    println!("m = {}, n = {}", m, n);
    println!("map = {:?}", map);
}

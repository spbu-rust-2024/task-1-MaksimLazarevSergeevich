use std::io;
use std::str::FromStr;

fn main() {

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Error");

    let mut numbers: Vec<isize> = input
        .split_whitespace()
        .map(|s| isize::from_str(s).unwrap())
        .collect();

    let mut i = 1;
    while i < numbers.len() {
        let mut j = i;
        while (j > 0) && (numbers[j - 1] > numbers[j]) {
            let useless = numbers[j];
            numbers[j] = numbers[j - 1];
            numbers[j - 1] = useless;
            j = j - 1;
        }
        i = i + 1;
    }
    for n in 0..numbers.len() {
        if n == (numbers.len() - 1) {
            print!("{}", numbers[n]);
        }else {
            print!("{} ", numbers[n]);
        }
    }
}


use std::io;

mod division;

fn main() {
    let input_line = read_line();
    let input_integer = string_to_integer(&input_line);
    let reciprocal= division::division(1,input_integer);
    println!("{}",reciprocal);
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
fn read_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

fn string_to_integer(input_string: &str) -> u64 {
    parse_input!(input_string, u64)
}
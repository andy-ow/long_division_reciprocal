fn main() {
    let n = 7u32;
    let reciprocal = reciprocal(n);
    println!("{:?}",reciprocal);
}

#[derive(Debug, std::cmp::PartialEq)]
struct Step {
    result: u64,
    quotient: u64,
    remainder: u64
}
fn divide_with_remainder(quotient:u64, dividor:u64) -> Step {
    let result = quotient / dividor;
    let remainder = quotient % dividor;
    Step {result, quotient, remainder}
}
fn reciprocal(dividor: u32) -> Vec<u64> {
    let mut dividor = dividor as u64;
    let mut quotient = 1u64;
    let mut answer = Vec::new();
    let mut n = 100;
    loop {
        n = n-1;
        let step = divide_with_remainder(quotient, dividor);
        answer.push(step.result);
        if step.remainder == 0 || n==0 {break}
        quotient = step.remainder * 10;
    }
    answer
}

#[cfg(test)]
mod tests {
    use crate::{divide_with_remainder, Step};

    #[test]
    fn divide_with_remainder_test() {
        let a = divide_with_remainder(11, 4);
        let b = Step { result: 2, quotient: 11, remainder: 3};
        assert_eq!(a,b);
    }
}
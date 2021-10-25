use std::collections::HashMap;

#[derive(Debug, Clone, std::cmp::PartialEq)]
struct Step {
    result: u64,
    quotient: u64,
    remainder: u64,
    index: usize,
}

fn divide_with_remainder(quotient:u64, dividor:u64, index:usize) -> Step {
    let result = quotient / dividor;
    let remainder = quotient % dividor;
    Step {result, quotient, remainder, index}
}

pub fn division(mut quotient: u64, dividor: u64) -> String {
    let mut answer = Vec::new();
    let mut history:HashMap<u64,Step> = HashMap::new();
    let mut index = 0;
    let mut bracket_position = 0;
    loop {
        let step = divide_with_remainder(quotient, dividor, index);
        if history.contains_key(&step.quotient) {
            bracket_position = history.get(&step.quotient).unwrap().index;
            break}
        history.insert(step.quotient,step.clone());
        answer.push(step.result);
        if step.remainder == 0 {break}
        quotient = step.remainder * 10;
        index += 1;
    }
    vector_to_string(answer, bracket_position)
}

fn vector_to_string(answer_vector: Vec<u64>, bracket_position: usize) -> String {
    let digits = ['0','1','2','3','4','5','6','7','8','9'];
    let mut answer = String::new();
    answer.push_str(&*answer_vector[0].to_string());
    answer.push('.');
    for i in 1..answer_vector.len() {
        if i == bracket_position {answer.push('(');}
        answer.push(digits.get(answer_vector[i] as usize).unwrap().to_owned());
    }
    if bracket_position != 0 {answer.push(')');}
    answer
}

#[cfg(test)]
mod tests {
    use crate::division;
    use crate::division::{divide_with_remainder, Step};

    #[test]
    fn divide_with_remainder_test() {
        let a = divide_with_remainder(11, 4, 5);
        let b = Step { result: 2, quotient: 11, remainder: 3, index: 5};
        assert_eq!(a,b);
    }
    #[test]
    fn divide_1_7() {
        let a ="0.(142857)";
        let b = division::division(1, 7);
        assert_eq!(a,b);
    }
}

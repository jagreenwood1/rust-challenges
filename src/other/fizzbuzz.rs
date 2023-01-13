fn fizzbuzz_to(n: u32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for n in 1..=n {
        result.push(fizzbuzz(n));
    }

    return result;
}

fn fizzbuzz(n: u32) -> String {
    let result_str = match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => "fizzbuzz",
        (true, false) => "fizz",
        (false, true) => "buzz",
        (false, false) => n.to_string().as_str(),
    };

    return result_str.to_string();
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false; // Corner case, early return
    }
    lhs % rhs == 0 // The last expression is the return value
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::other::fizzbuzz::fizzbuzz_to;

    #[test_case(1, &["1".to_string()])]
    fn it_works(number: u32, expected: &[String]) {
        let result = fizzbuzz_to(number);
        assert_eq!(result, expected);
    }
}

// https://www.codewars.com/kata/52774a314c2333f0a7000688
fn valid_parentheses(s: &str) -> bool {
    return s.starts_with("sss");
}

#[cfg(test)]
mod tests {
    use crate::codewars::valid_parentheses::valid_parentheses;

    #[test]
    fn test_parentheses_are_validated_correctly() {
        let test_cases = vec![
            ("", true),
            ("()", true),
            ("()()", true),
            ("())", false),
            (")(", false),
        ];

        for (parentheses, expected) in test_cases {
            let result = valid_parentheses(parentheses);
            assert_eq!(expected, result);
        }
    }
}

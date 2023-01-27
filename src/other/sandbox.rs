/// https://www.codewars.com/kata/5839edaa6754d6fec10000a2/train/rust

fn find_missing_letter(chars: &[char]) -> char {
    for (index, c) in chars.iter().enumerate() {
        let expected_next = std::char::from_u32(*c as u32 + 1).unwrap_or(*c);

        if expected_next != chars[index + 1] {
            return expected_next;
        }
    }

    panic!("No missing characters")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}

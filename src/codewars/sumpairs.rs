// Given a list of integers and a single sum value, return the first two values
// (parse from the left please) in order of appearance that add up to form the sum.

// If there are two or more pairs with the required sum, the pair whose second
// element has the smallest index is the solution.

use std::collections::HashSet;

fn sum_pairs(ints: &[i8], sum: i8) -> Option<(i8, i8)> {
    for num_a in ints {
        for num_b in ints {
            if num_a + num_b == sum {
                return Some((*num_a, *num_b));
            }
        }
    }

    None
}

fn sum_pairs_original(ints: &[i8], sum: i8) -> Option<(i8, i8)> {
    let mut seen: HashSet<i8> = HashSet::new();
    for number in ints {
        let difference = sum - number;
        if seen.contains(&difference) {
            return Some((difference, *number));
        }
        seen.insert(*number);
    }
    None
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::sumpairs::sum_pairs;

    #[test_case(&[1, 4, 8, 7, 3, 15], 8, (1, 7))]
    #[test_case(&[1, -2, 3, 0, -6, 1], -6, (0, -6))]
    fn should_return_first_two_pairs_that_sum_to_given_number(
        ints: &[i8],
        number: i8,
        expected: (i8, i8),
    ) {
        let actual = sum_pairs(ints, number);
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test_case(&[], 5)]
    #[test_case(&[1, 2, 3], 0)]
    fn should_return_empty_option_when_no_pairs_exist(ints: &[i8], number: i8) {
        let actual = sum_pairs(ints, number);
        assert!(actual.is_none());
    }
}

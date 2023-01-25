fn high_and_low_a(numbers: &str) -> String {
    let mut high = -f64::INFINITY;
    let mut low = f64::INFINITY;

    numbers.split(' ').for_each(|num_str| {
        let num_int = num_str.parse::<f64>().unwrap();
        if num_int > high {
            high = num_int
        }

        if num_int < low {
            low = num_int
        }
    });

    format!("{high} {low}")
}

fn high_and_low(numbers: &str) -> String {
    use std::cmp;

    let (high, low) = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::min_value(), i32::max_value()), |(max, min), x| {
            (cmp::max(max, x), cmp::min(min, x))
        });
    format!("{high} {low}")
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}

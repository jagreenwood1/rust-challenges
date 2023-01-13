/**
 * In this kata you have to correctly return who is the "survivor", ie: the last element of a Josephus permutation.

Basically you have to assume that n people are put into a circle and that they are eliminated in steps of k elements, like this:

josephus_survivor(7,3) => means 7 people in a circle;
one every 3 is eliminated until one remains
[1,2,3,4,5,6,7] - initial sequence
[1,2,4,5,6,7] => 3 is counted out
[1,2,4,5,7] => 6 is counted out
[1,4,5,7] => 2 is counted out
[1,4,5] => 7 is counted out
[1,4] => 5 is counted out
[4] => 1 counted out, 4 is the last element - the survivor!
The above link about the "base" kata description will give you a more thorough insight about the origin of this kind of permutation, but basically that's all that there is to know to solve this kata.

Notes and tips: using the solution to the other kata to check your function may be helpful, but as much larger numbers will be used, using an array/list to compute the number of the survivor may be too slow; you may assume that both n and k will always be >=1.
 */

fn josephus_survivor_original(person_count: i32, skip: i32) -> i32 {
    (1..person_count + 1).fold(0, |acc, cur| (acc + skip) % cur) + 1
}

fn josephus_survivor(person_count: i32, skip: i32) -> i32 {
    let range_of_people_ints = 1..person_count + 1;
    let last_person_index = range_of_people_ints.fold(0, |acc, cur| (acc + skip) % cur);

    return last_person_index + 1;
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::josephus::josephus_survivor;

    #[test_case(7, 3, 4)]
    #[test_case(11, 19, 10)]
    #[test_case(1, 300, 1)]
    #[test_case(2, 300, 1)]
    #[test_case(7, 300, 7)]
    #[test_case(300, 300, 265)]
    fn test_josephus_survivor(n: i32, k: i32, expected: i32) {
        let actual = josephus_survivor(n, k);
        assert_eq!(actual, expected);
    }
}

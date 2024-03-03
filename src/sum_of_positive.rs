// You get an array of numbers, return the sum of all of the positives ones.

// Example [1,-4,7,12] => 1 + 7 + 12 = 20

// Note: if there is nothing to sum, the sum is default to 0.
// https://www.codewars.com/kata/5715eaedb436cf5606000381

fn positive_sum(slice: &[i32]) -> i32 {
    slice
        .into_iter()
        .fold(0, |acc, number| match number.is_positive() {
            true => acc + number,
            false => acc,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
        assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
    }
}

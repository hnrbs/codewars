// # Convert number to reversed array of digits

// Given a random non-negative number, you have to return the
// digits of this number within an array in reverse order.

// # Example(Input => Output):

// ```
// 35231 => [1,3,2,5,3]
// 0 => [0]
// ```

// https://www.codewars.com/kata/5583090cbe83f4fd8c000051/

fn digitize(n: u64) -> Vec<u8> {
    let number = n.to_string();
    let rev_chars = number.chars().rev();
    rev_chars
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(348597), vec![7, 9, 5, 8, 4, 3]);
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
    }
}

// This time no story, no theory. The examples below show you how to write function accum:

// Examples:
// accum("abcd") -> "A-Bb-Ccc-Dddd"
// accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
// accum("cwAt") -> "C-Ww-Aaa-Tttt"
// The parameter of accum is a string which includes only letters from a..z and A..Z.

// https://www.codewars.com/kata/5667e8f4e3f572a8f2000039

fn times(position: usize, char: char) -> String {
    let mut part = String::with_capacity(position);
    part.push(char.to_ascii_uppercase());

    for _ in 0..position {
        part.push(char.to_ascii_lowercase())
    }

    part
}

fn accum(s: &str) -> String {
    s.chars()
        .enumerate()
        .fold(String::new(), |acc, (position, char)| {
            let x = times(position, char);

            if acc.is_empty() {
                return x;
            }

            acc + "-" + &x
        })
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html
#[test]
fn basic_tests() {
    assert_eq!(
        accum("ZpglnRxqenU"),
        "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("NyffsGeyylB"),
        "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
    );
    assert_eq!(
        accum("MjtkuBovqrU"),
        "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("EvidjUnokmM"),
        "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
    );
    assert_eq!(
        accum("HbideVbxncC"),
        "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
    );
}

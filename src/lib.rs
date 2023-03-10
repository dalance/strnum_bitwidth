pub fn bitwidth(s: &str, base: u32) -> Option<usize> {
    let s = s.trim_start_matches("0");
    let s = s.replace("_", "");
    let s = s.to_ascii_lowercase();

    match base {
        2 => Some(s.chars().count()),
        4 => {
            let mut width = s.chars().count() * 2;
            match s.chars().next() {
                Some('1') => width -= 1,
                _ => (),
            }
            Some(width)
        }
        8 => {
            let mut width = s.chars().count() * 3;
            match s.chars().next() {
                Some('1') => width -= 2,
                Some('2') => width -= 1,
                Some('3') => width -= 1,
                _ => (),
            }
            Some(width)
        }
        16 => {
            let mut width = s.chars().count() * 4;
            match s.chars().next() {
                Some('1') => width -= 3,
                Some('2') => width -= 2,
                Some('3') => width -= 2,
                Some('4') => width -= 1,
                Some('5') => width -= 1,
                Some('6') => width -= 1,
                Some('7') => width -= 1,
                _ => (),
            }
            Some(width)
        }
        32 => {
            let mut width = s.chars().count() * 5;
            match s.chars().next() {
                Some('1') => width -= 3,
                Some('2') => width -= 2,
                Some('3') => width -= 2,
                Some('4') => width -= 1,
                Some('5') => width -= 1,
                Some('6') => width -= 1,
                Some('7') => width -= 1,
                Some('8') => width -= 1,
                Some('9') => width -= 1,
                Some('a') => width -= 1,
                Some('b') => width -= 1,
                Some('c') => width -= 1,
                Some('d') => width -= 1,
                Some('e') => width -= 1,
                Some('f') => width -= 1,
                _ => (),
            }
            Some(width)
        }
        _ => {
            if let Ok(number) = u128::from_str_radix(&s, base) {
                Some(number.ilog2() as usize + 1)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary() {
        assert_eq!(bitwidth("1", 2), Some(1));
        assert_eq!(bitwidth("11", 2), Some(2));
        assert_eq!(bitwidth("10101", 2), Some(5));
        assert_eq!(bitwidth("010101", 2), Some(5));
        assert_eq!(bitwidth("0_1_01_0_1", 2), Some(5));
    }

    #[test]
    fn octal() {
        assert_eq!(bitwidth("1", 8), Some(1));
        assert_eq!(bitwidth("11", 8), Some(4));
        assert_eq!(bitwidth("12451", 8), Some(13));
        assert_eq!(bitwidth("013171", 8), Some(13));
    }

    #[test]
    fn decimal() {
        assert_eq!(bitwidth("1", 10), Some(1));
        assert_eq!(bitwidth("11", 10), Some(4));
        assert_eq!(bitwidth("19171", 10), Some(15));
        assert_eq!(bitwidth("014344", 10), Some(14));
    }

    #[test]
    fn hex() {
        assert_eq!(bitwidth("1", 16), Some(1));
        assert_eq!(bitwidth("11", 16), Some(5));
        assert_eq!(bitwidth("1f1F1", 16), Some(17));
        assert_eq!(bitwidth("01a1A1", 16), Some(17));
    }
}

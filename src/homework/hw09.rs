pub fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    // Зведення зсуву до діапазону [0, len)
    let shift = ((n % len as isize + len as isize) % len as isize) as usize;

    let chars: Vec<char> = s.chars().collect();
    let rotated: Vec<char> = chars[len - shift..]
        .iter()
        .chain(chars[..len - shift].iter())
        .cloned()
        .collect();

    rotated.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotate2(s: &str, n: &isize) -> String {
        rotate(s.to_string(), *n)
    }

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(
                rotate2(s, n),
                exp.to_string()
            )
        });
    }
}


pub fn has_repeated_digits(s: &str) -> bool {
    let len = s.len();
    if len % 2 != 0 { return false; }
    let half = len / 2;
    s[..half] == s[half..]
}

pub fn has_some_repeated_digits_slow(s: &str) -> bool {
    let len = s.len();
    if len < 2 {
        return false;
    }
    for pattern_len in 1..=len/2 {
        if len % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            let repetitions = len / pattern_len;
            // Is repeating the pattern gives us the original string
            if pattern.repeat(repetitions) == s {
                return true;
            }
        }
    }
    false
}

pub fn has_some_repeated_digits_fast(s: &str) -> bool {
    let doubled = format!("{}{}", s, s);
    doubled[1..doubled.len()-1].contains(s)
}

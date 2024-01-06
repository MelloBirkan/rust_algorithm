pub(crate) fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut p_start = 0;
    let mut p_end = chars.len().saturating_sub(1);

    while p_start < p_end {
        if chars[p_start] != chars[p_end] {
            return false;
        }
        p_start += 1;
        p_end -= 1;
    }
    true
}

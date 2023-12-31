pub fn longest_word(s: String) -> String {
    let mut longest = String::new();
    let words: Vec<&str> = s.split_whitespace().collect();
    for word in words {
        if word.len() > longest.len() {
            longest = word.to_string();
        }
    }
    longest
}
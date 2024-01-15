pub fn is_isogram(word: &str) -> bool {
    use std::collections::HashSet;

    let mut seen = HashSet::new();

    for ch in word.to_lowercase().chars() {
        // Se o caractere for uma letra e já foi visto, não é um isograma
        if ch.is_alphabetic() && !seen.insert(ch) {
            return false;
        }
    }

    true
}
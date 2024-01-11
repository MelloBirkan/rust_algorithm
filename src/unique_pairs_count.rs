use std::collections::HashSet;

pub fn unique_pairs_count(k: i32, arr: Vec<i32>) -> i32 {
    let mut seen = HashSet::new();
    let mut pairs = HashSet::new();

    for &number in &arr {
        if seen.contains(&(k - number)) {
            // Insert a sorted tuple to ensure uniqueness
            let mut pair = vec![number, k - number];
            pair.sort_unstable(); // Sorting to ensure (a, b) and (b, a) are not counted twice
            pairs.insert(pair);
        }
        seen.insert(number);
    }

    pairs.len() as i32
}

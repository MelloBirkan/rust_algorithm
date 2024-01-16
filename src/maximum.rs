pub fn maximum(_numbers: &[i64]) -> i64 {
    // your code goes here!
    let mut max = _numbers[0];
    for n in _numbers.iter() {
        if *n > max {
            max = *n;
        }
    }
    max
}

fn maximum_or_zero(numbers: &[u32]) -> u32 {
    *numbers.iter().max().unwrap_or(&0)
}

pub fn fact(n: i32) -> i32 {
    if n == 0 {
        return 1
    }
    return n * fact(n - 1);
}
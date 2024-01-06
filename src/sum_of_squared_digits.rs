pub fn sum_of_Squared_digits(n: &i32)  -> i32{
    let mut total_sum = 0;
    let mut n = *n;

    while n > 0 {
        let digit = n % 10;
        n /= 10;
        total_sum += (digit * digit) as i32;
    }
    total_sum
}
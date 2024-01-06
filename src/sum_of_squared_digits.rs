pub fn sum_of_Squared_digits(n: &mut f32)  -> i32{
    let mut total_sum = 0;

    while *n > 0f32 {
        let digit = *n % 10f32;
        *n = (*n as f32/10f32).floor();
        total_sum += (digit * digit) as i32;
    }
    total_sum
}
use crate::sum_of_squared_digits::sum_of_Squared_digits;

pub fn is_happy_number(n: &i32) -> bool {
    let mut p_slow = *n;
    let mut p_fast = sum_of_Squared_digits(&n);

    while p_fast != 1 && p_fast != p_slow {
        p_slow = sum_of_Squared_digits(&p_slow);
        p_fast = sum_of_Squared_digits(&sum_of_Squared_digits(&p_fast));
    }
    if p_fast == 1 {
        return true
    }
    false
}
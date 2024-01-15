fn reverse(my_vec: &mut Vec<i64>)-> &mut Vec<i64>{
    let mut p_1 = 0;
    let mut p_2 = my_vec.len() - 1;
    while p_1 < p_2 {
        let temp = my_vec[p_1];
        my_vec[p_1] = my_vec[p_2];
        my_vec[p_2] = temp;
        p_1 += 1;
        p_2 -= 1;
    }
    my_vec
}
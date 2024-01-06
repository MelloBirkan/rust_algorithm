pub fn insertion_sort(arr: &mut [i32]) {
    for j in 1..arr.len() {
        let chave = arr[j];
        let mut i: isize = j as isize - 1;
        while i >= 0 && arr[i as usize] > chave {
            arr[(i + 1) as usize] = arr[i as usize];
            i = i - 1;
        }
        arr[(i + 1) as usize] = chave;
    }
}
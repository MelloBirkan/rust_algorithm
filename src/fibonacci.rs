pub fn fibonacci(term: u32) -> u32 {
    match term {
        0 =>  0,
        1 =>  1,
        _ => fibonacci(term-1) + fibonacci(term-2),
    }
}

// esse programa retorna um vetor com os n primeiros numeros da sequencia de fibonacci {
fn fibs(count: usize) -> Vec<u32> {
    let mut fib: Vec<u32> = Vec::new();
    for n in 0..count {
        fib.push(fibonacci(n as u32));
    }
    fib
}

fn fibs_iterative(count: usize) -> Vec<u32> {
    let mut result = vec![];
    let mut x = 1;
    let mut y = 1;
    let mut i = 0;

    while i < count {
        result.push(x);
        let next = x + y;
        x = y;
        y = next;
        i += 1;
    }

    result
}

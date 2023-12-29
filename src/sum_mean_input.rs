use std::io;

pub fn sum_mean_input() {
    /* Usamos slice (&[i32]) pois é mais comum e idiomático em Rust, isso torna a função mais
    geral e capaz de aceitar qualquer fatia de um vetor (não só Vec).
     */
    fn sum_mean(numbers: &[i32]) -> (i32, i32) {
        if numbers.is_empty() {
            return (0, 0);
        }
        let sum: i32 = numbers.iter().sum();
        let mean = sum / numbers.len() as i32;
        (sum, mean)
    }

    let mut input = String::new();
    let mut numbers:Vec<i32> = Vec::new();
    while input.trim() != "stop".to_string() {
        if numbers.len() > 1 {
            let mut input = String::new();
            println!("Deseja somar e ver a média dos numeros {:?} ? (s/n)", numbers);
            io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
            if input.trim() == "s" {
                let (sum, mean) = sum_mean(&numbers);
                println!("A some dos números {:?} é {} e a média é {}", numbers, sum,
                         mean);
            }
        }
        input.clear();
        println!("Digite um número para somar: (stop para sair)");
        io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
        match input.trim().parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => {
                println!("Por favor, digite um número válido.");
                continue;
            }
        }
    }
}
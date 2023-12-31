mod sum_mean_input;
mod recursive_factorial;
mod longest_word;

use recursive_factorial::fact;
use sum_mean_input::sum_mean_input;

fn main() {
    sum_mean_input::sum_mean_input();
    println!("o Fatorial de 5 é {}", fact(5));
    println!("A maior palavra é {}", longest_word::longest_word("O rato roeu a roupa do rei de roma".to_string()));
}

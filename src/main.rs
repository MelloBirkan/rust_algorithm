mod sum_mean_input;
mod recursive_factorial;

use recursive_factorial::fact;

fn main() {
    sum_mean_input::sum_mean_input();
    println!("o Fatorial de 5 é {}", fact(5));
}

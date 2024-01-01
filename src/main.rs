mod sum_mean_input;
mod recursive_factorial;
mod longest_word;
mod shapes_area;

use recursive_factorial::fact;
use sum_mean_input::sum_mean_input;
use longest_word::longest_word;

use shapes_area::{Shape, largest_shape};

fn main() {
    sum_mean_input::sum_mean_input();
    println!("o Fatorial de 5 é {}", fact(5));
    println!("A maior palavra é {}", longest_word("O rato roeu a roupa do rei de roma".to_string()));
    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Square(3.0),
        Shape::Triangule(2.0, 3.0),
    ];
    largest_shape(&shapes);
}

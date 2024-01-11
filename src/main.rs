mod sum_mean_input;
mod recursive_factorial;
mod longest_word;
mod shapes_area;
mod insertion_sort;
mod palindrome;
mod sum_of_squared_digits;
mod is_happy_number;
mod fibonacci;
mod unique_pairs_count;

use recursive_factorial::fact;
use sum_mean_input::sum_mean_input;
use longest_word::longest_word;
use shapes_area::{Shape, largest_shape};
use insertion_sort::insertion_sort;
use palindrome::is_palindrome;
use sum_of_squared_digits::sum_of_Squared_digits;
use crate::is_happy_number::is_happy_number;
use crate::unique_pairs_count::unique_pairs_count;

fn main() {
    let mut arr = [16, 3, 6, 8, 0, 10];
    println!("o Fatorial de 5 é {}", fact(5));
    println!("A maior palavra é {}", longest_word("O rato roeu a roupa do rei de roma".to_string()));
    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Square(3.0),
        Shape::Triangule(2.0, 3.0),
    ];
    largest_shape(&shapes);
    // sum_mean_input();
    insertion_sort(&mut arr);
    println!("{:?}", arr);
    println!("{}",is_palindrome(&String::from("amlma")));
    println!("{}", is_happy_number(&100));

}

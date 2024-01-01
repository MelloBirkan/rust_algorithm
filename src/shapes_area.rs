pub enum Shape {
    Circle(f64),
    Square(f64),
    Triangule(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        let area = match self {
            Shape::Triangule(base, height) => (base * height) / 2.0,
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
        };
        area
    }

    fn name(&self) -> String {
        match self {
            Shape::Triangule(_, _) => String::from("Triangulo"),
            Shape::Circle(_) => String::from("Circulo"),
            Shape::Square(_) => String::from("Quadrado"),
        }
    }
}

pub fn largest_shape(v: &[Shape]) {
    let mut maior: f64 = 0.0;
    let mut maior_nome = String::new();

    v.iter().for_each(
        (|shape| {
            let area = shape.area();
            if area > maior {
                maior = area;
                maior_nome = shape.name();
            }
        }),
    );
    println!(
        "O shape que tem maior area é o {}, com uma area total de: {:.2}",
        maior_nome, maior
    )
}

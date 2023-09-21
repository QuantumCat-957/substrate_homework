trait Area {
    fn calculate_area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn calculate_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn calculate_area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Area>(shape: T) {
    let area = shape.calculate_area();
    println!("The area is: {:.2}", area);
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle {
        base: 4.0,
        height: 6.0,
    };
    let square = Square { side: 3.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}

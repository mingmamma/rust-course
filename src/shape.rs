enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64, f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match &self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(a, b, c) => ((a+b+c)/2.0*((a+b+c)/2.0-a)*((a+b+c)/2.0-b)*((a+b+c)/2.0-c)).sqrt()
        }
    }
}

fn largest_shape(shape_vec: &Vec<Shape>) -> &Shape {
   
    assert!(!shape_vec.is_empty());
   
    let mut max_area= 0.0;
    let mut max_shape = &shape_vec[0];

    for shape in shape_vec {
        if shape.area() > max_area {
            max_shape = shape;
            max_area = shape.area();
        }
    }
    max_shape
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(2.0, 2.0, 2.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(a, b, c) => ((a+b+c)/2.0*((a+b+c)/2.0-a)*((a+b+c)/2.0-b)*((a+b+c)/2.0-c)).sqrt()
        })
        .sum();

    println!("Total area: {} sq. units", total_area);

    let largest_shape = largest_shape(&shapes);

    println!("Largest area: {} sq. units", largest_shape.area())
}
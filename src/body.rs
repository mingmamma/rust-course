fn main() {
    let message: &str = "Name: Max, Weight: ";
    let weight = 130.0;

    let kilo = weight / 2.2;
    println!("{}{}", message, kilo);

    let mut message = String::from("Name: Max, Height: ");
    message.clear();
    let mut height = 180;
    height = height - 20;
    println!("{}{}", message, height);

    let result = if height > 180 {
        "tall"
    } else if height > 170 {
        "average"
    } else {
        "short"
    };
    
    println!("Result: {}", result)
}

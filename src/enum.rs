#[derive(Debug)]
enum Grape {
    Carbernet,
    Tannet,
    Merlot,
}

struct Wine {
    name: String,
    grape: Grape
}

impl Wine {
    fn get_wine_popularity(&self) -> () {
        match &self.grape {
            Grape::Merlot => println!("Popular Merlot wine!"),
            _ => ()            
        }
    }
}

fn main() {
    let wine_1 = Wine {
        name: "merlot wine".to_string(),
        grape: Grape::Merlot,
    };
    
    wine_1.get_wine_popularity();
}
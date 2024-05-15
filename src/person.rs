#[derive(Debug)]
struct Person {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}

impl Person {
    fn new(first_name: String, middle_name: Option<String>, last_name: String) -> Self {
        Self {
            first_name: first_name,
            middle_name: middle_name,
            last_name: last_name,
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", &self.first_name, &self.last_name)
    }

    fn build_full_name(&self) -> String {
        let mut full_name = String::new();
        full_name.push_str(&self.first_name);
        full_name.push_str(" ");

        match &self.middle_name {
            Some(middle_name) => full_name.push_str(middle_name),
            None => (),
        }

        full_name.push_str(" ");
        full_name.push_str(&self.last_name);
        full_name
    }
}

fn main() {
    let john = Person::new("John".to_string(), None, "Doe".to_string());

    println!("{:?}", &john);

    println!("{:?}", john.get_full_name());

    let james = Person {
        first_name: String::from("James"),
        middle_name: Some(String::from("Oliver")),
        last_name: String::from("Smith"),
    };

    assert_eq!(james.build_full_name(), "James Oliver Smith");
}

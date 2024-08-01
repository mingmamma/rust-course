#![allow(dead_code, unused_variables)]
#![feature(error_iter)]

trait AsJson {
    fn as_json(&self) -> String;
}

fn send_data_as_json(value: &impl AsJson) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
            self.name, self.color, self.likes_petting
        )
    }
}

struct Cat {
    name: String,
    sharp_claws: bool,
}

impl AsJson for Cat {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "cat", "name": "{}", "hasSharpClaws": {} }}"#,
            self.name, self.sharp_claws
        )
    }
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct InnerError;

#[derive(Debug)]
struct ErrorWrap(Option<Box<dyn Error + 'static>>);
// struct ErrorWrap(InnerError);

impl fmt::Display for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InnerError")
    }
}

impl fmt::Display for ErrorWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorWrap")
    }
}

impl Error for InnerError {}

impl Error for ErrorWrap {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let as_ref = self.0.as_ref();
        as_ref.map(|ref_boxed_err: &Box<dyn Error + 'static>| ref_boxed_err.as_ref())
        // Some(&self.0)
    }
}


fn main() {
    let laura = Person {
        name: String::from("Laura"),
        age: 31,
        favorite_fruit: String::from("apples"),
    };

    let fido = Dog {
        name: String::from("Fido"),
        color: String::from("Black"),
        likes_petting: true,
    };

    let kitty = Cat {
        name: String::from("Kitty"),
        sharp_claws: false,
    };

    send_data_as_json(&laura);
    send_data_as_json(&fido);
    send_data_as_json(&kitty);

    {
    let test_err = ErrorWrap(Some(Box::new(InnerError)));
    // let test_err = ErrorWrap(InnerError);

    let ref_dyn_error : Box<dyn Error> = test_err.into(); // or
    // let ref_dyn_error: &dyn Error = &test_err as &(dyn Error);

    // explicitly casting to a value of &dyn Error, on which the sources is implemented for
    // let ref_dyn_error: &dyn Error = &test_err; 
    
    let mut err_source_iter = ref_dyn_error.sources();

    assert_eq!("B".to_string(), err_source_iter.next().unwrap().to_string());
    assert_eq!("A".to_string(), err_source_iter.next().unwrap().to_string());
    assert!(err_source_iter.next().is_none());
    assert!(err_source_iter.next().is_none());
    }
}

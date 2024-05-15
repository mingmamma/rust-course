// a function that takes a string of words separated by spaces and
// returns the first word it finds in that string. If the function
// doesn’t find a space in the string, the whole string must be one word,
//so the entire string should be returned
fn get_first_word(input_str: &str) -> &str {
    let bytes = input_str.as_bytes();

    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input_str[..idx];
        }
    }

    &input_str[..]
}

fn main() {
    let my_string = String::from("hello world");

    // works on reference of String slice
    let first_word = get_first_word(&my_string[..6]);

    println!("{}", first_word);

    let first_word = get_first_word(&my_string[..]);
    // works on reference of String
    let first_word = get_first_word(&my_string);

    let my_string_literal = "hello world";

    // works on reference of &str slice
    let first_word = get_first_word(&my_string_literal[..6]);
    let first_word = get_first_word(&my_string_literal[..]);
    // works on reference of String
    let first_word = get_first_word(&my_string_literal);

    println!("{}", first_word);
}

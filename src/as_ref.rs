fn main() {
    let text: Option<String> = Some("Hello, world!".to_string());
    
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume "that" Option with `map`, leaving `text` on the stack.
    let text_length_1 = text.as_ref().map(|s| s.len());
    println!("still can access the original text variable: {text:?}");
    
    // Reminder: the length is w.r.t bytes
    if let Some(text_length_int) = text_length_1 {
        println!("original text String was {} long", text_length_int);
    }

    if let Some(text_length_int_2) = text.map(|s| s.len()) {
        println!("original text String was {} long, obtained any way", text_length_int_2);
    }
    
    // text variable is not available anymore since consumed by the map call
    // println!("Trying to access the original text varaibble {}", text);

}
fn main() {
    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length_1 = text.as_ref().map(|s| s.len());
    println!("still can print text: {text:?}");

    let text_length_2 = text.map(|s| s.len());
    // text variable is not available anymore since consumed by the map call
}
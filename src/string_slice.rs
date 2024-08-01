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

    {
        let test_unmovable: std::pin::Pin<Box<Unmovable>> = Unmovable::new();

        // The inner pointee `Unmovable` struct will now never be allowed to move.
        // Meanwhile, we are free to move the pointer around.
        let still_unmoved: std::pin::Pin<Box<Unmovable>> = test_unmovable;
        assert_eq!(still_unmoved.slice, std::ptr::NonNull::from(&still_unmoved.data));

        // We cannot mutably dereference a `Pin<Ptr>` unless the pointee is `Unpin` or we use unsafe.
        // Since our type doesn't implement `Unpin`, this will fail to compile.
        // let mut new_unmoved = Unmovable::new();
        // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
    }
}

/// This is a self-referential struct because `self.slice` points into `self.data`.
struct Unmovable {
    /// Backing buffer.
    data: [u8; 64],
    /// Points at `self.data` which we know is itself non-null. Raw pointer because we can't do
    /// this with a normal reference.
    slice: std::ptr::NonNull<[u8]>,
    _pin: std::marker::PhantomPinned,
}

impl Unmovable {
    /// Create a new `Unmovable`.
    ///
    /// To ensure the data doesn't move we place it on the heap behind a pinning Box.
    /// Note that the data is pinned, but the `Pin<Box<Self>>` which is pinning it can
    /// itself still be moved. This is important because it means we can return the pinning
    /// pointer from the function, which is itself a kind of move!
    fn new() -> std::pin::Pin<Box<Unmovable>> {
    // fn new() -> std::pin::Pin<&mut Unmovable> {
        
        let temp_arr_ref: &[u8; 0]  = &[];
        let res: Unmovable = Unmovable {
            data: [0; 64],
            // We only create the pointer once the data is in place
            // otherwise it will have already moved before we even started.
            // We can supply an argument to type &[u8; 0], or read as "shared reference of array
            // type [u8; 0]", whereas the parameter type is [u8], read as "slice type of u8",
            // is because array types coerce into slice types, whereby "coersion" refers to
            // type conversion that is implicit and automatic
            // slice: NonNull::from(&[]),
            slice: std::ptr::NonNull::from(temp_arr_ref),
            _pin: std::marker::PhantomPinned,
        };

        // res.slice = std::ptr::NonNull::from(&res.data);
        // let pinned_res: std::pin::Pin<&mut Unmovable> = std::pin::pin!(res);
        // pinned_res

        // First we put the data in a box, which will be its final resting place
        let mut boxed_res: Box<Unmovable> = Box::new(res);

        // Then we make the slice field point to the proper part of that boxed data.
        // From now on we need to make sure we don't move the boxed data.
        boxed_res.slice = std::ptr::NonNull::from(&(boxed_res.data));

        // To do that, we pin the data in place by pointing to it with a pinning
        // (`Pin`-wrapped) pointer.
        //
        // `Box::into_pin` makes existing `Box` pin the data in-place without moving it,
        // so we can safely do this now *after* inserting the slice pointer above, but we have
        // to take care that we haven't performed any other semantic moves of `res` in between.
        let pinned_boxed_res: std::pin::Pin<Box<Unmovable>> = Box::into_pin(boxed_res);

        // Now we can return the pinned (through a pinning Box) data
        pinned_boxed_res
    }
}
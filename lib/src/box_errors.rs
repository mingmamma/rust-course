//https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html#defining-an-error-type

// program for demoing boxing different errors
// likely encountered when doubling the first
// element of a vector comparable to integer
use std::{error, fmt::{self, Formatter}};

// aliasing to shorter type
// type alias c.f. https://doc.rust-lang.org/book/ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases

// Boxed errors makes sure of the trait object s.t it accepts all type of errors that implemented the Error trait
// dyn keyword introduce the trait object that allows values of different types implementing a common trait
// Dynamic dispatch with runtime cost follows to look up the method to call 
// c.f. https://doc.rust-lang.org/book/ch17-02-trait-objects.html#defining-a-trait-for-common-behavior
// https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch
// Trait object must be used with some form of pointer, and hence Box in this case

// The from() function for the FROM trait, implemented by error:ERROR must have been used to convert
// a specific error to Box<dyn error:Error>
// c.f https://doc.rust-lang.org/std/error/trait.Error.html#impl-From%3CE%3E-for-Box%3Cdyn+Error%3E
// https://doc.rust-lang.org/std/convert/trait.From.html
type ResultA<T> = std::result::Result<T, Box<dyn error::Error>>;

fn print_result(result: ResultA<u32>) -> ()  {
    match result {
        Ok(num) => println!("{}", num),
        Err(e) => println!("{}", e),
    }
}

// demo 2 custom errors: empty vector s.t. no first element to double or non-parsable first element not able to be doubled
fn double_first_element(vec: Vec<&str>) ->  ResultA<u32> {
    // ok_or takes None returned by first() and gives Err(err) where err is supplied to be EmptyVecError
    let first_element = vec.first().ok_or(EmptyVecError)?;
    // map_err take an closure as agument. The closure supplied takes the Err value returned by parse::<u32>(),
    // disregard it, and return our custom UnparsableFirstElementError
    let parsed_num = first_element.parse::<u32>().map_err(|_| UnparsableFirstElementError)?;
    Ok(parsed_num * 2)
}

// https://doc.rust-lang.org/1.39.0/std/error/trait.Error.html
// Errors must describe themselves through the Display and Debug traits
// Hence first required to derive the Debug trait for our custom EmptyVecError
#[derive(Debug)]
struct EmptyVecError;

// And hence implemeneting the Display trait for the custom EmptyVecError
impl fmt::Display for EmptyVecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("First element of an empty vector can not be doubled")
    }
}

impl error::Error for EmptyVecError {}

#[derive(Debug)]
struct UnparsableFirstElementError;

impl fmt::Display for UnparsableFirstElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "First element is unparsable to be doubled")
    }
}

impl error::Error for UnparsableFirstElementError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_vector() {
        assert!(matches!(
            double_first_element(vec![]),
            Err(_)
        ));
    }

    #[test]
    fn test_vector_not_int_parsable() {
        assert!(matches!(
            double_first_element(vec!["int"]),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_vector() {
        assert!(double_first_element(vec!["3","1","2"]).is_ok());
        assert_eq!(
            double_first_element(vec!["3","1","2"]).unwrap(),
            6
        );        
    }

}
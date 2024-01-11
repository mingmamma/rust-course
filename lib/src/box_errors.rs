// program for demoing boxing different errors
// likely encountered when doubling the first
// element of a vector comparable to integer
use std::{error, fmt};

// aliasing to shorter type
type ResultA<T> = std::result::Result<T, Box<dyn error::Error>>;

fn print_result(result: ResultA<u32>) -> ()  {
    match result {
        Ok(num) => println!("{}", num),
        Err(e) => println!("{}", e),
    }
}

fn double_first_element(vec: Vec<&str>) ->  ResultA<u32> {
    let first_element = vec.first().ok_or(EmptyVecError)?;
    let parsed_num = first_element.parse::<u32>()?;
    Ok(parsed_num * 2)
}

#[derive(Debug)]
struct EmptyVecError;

// required to impl Error for EmptyVecError
impl fmt::Display for EmptyVecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid empty vector")
    }
}

impl error::Error for EmptyVecError {}

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
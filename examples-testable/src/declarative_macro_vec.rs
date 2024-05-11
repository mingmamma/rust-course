// c.f. https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming
//  indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope
#[macro_export]
macro_rules! vec {
    ($( $x: expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    ($elem:literal;$repetition:literal) => {
        {
            let mut temp_vec = Vec::new();
            for _ in 0..$repetition {
                temp_vec.push($elem)
            }
            temp_vec
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macro_given_list() {
        let test_vec = vec![1,2,3];
        
        let mut test_vec_expected = Vec::with_capacity(3);
        test_vec_expected.push(1);
        test_vec_expected.push(2);
        test_vec_expected.push(3);

        assert_eq!(test_vec, test_vec_expected);
    }

    #[test]
    fn test_macro_given_element_and_repetition() {
        let test_vec = vec![1;3];

        let mut test_vec_expected = Vec::with_capacity(3);
        test_vec_expected.push(1);
        test_vec_expected.push(1);
        test_vec_expected.push(1);

        assert_eq!(test_vec, test_vec_expected);

    }
}  
#![allow(unused)]
macro_rules! my_vec {
    
    // the basic input pattern of either no input or input of one expresion
    // as a starting point for the implementation, which is extended to a 
    // pattern that can handle arbitrarily many input expressions separated
    // by comma
    // () => {
    //     Vec::new()
    // };
    // ($e:expr) => {{
    //     let mut new_vec = Vec::new();
    //     new_vec.push($e);
    //     new_vec
    // }};
    
    // the input pattern of arbitrarily many expression sperated by comma as
    // elements to the vec, with a comma following the last element optionally
    // e.g. my_vec![2, 5, 8] or my_vec![1, 4, 7,]
    ($($e:expr),*$(,)?) => {{
        let mut new_vec = Vec::new();
        $(new_vec.push($e);)*
        new_vec
    }};
    // the input pattern of an expression as the element of vec, further specified with
    // the number of times for it to be repeated, separated by semicolon 
    // e.g. my_vec![1;3]
    ($e:literal;$rep:literal) => {{
        let mut new_vec = Vec::new();
        for _ in 0..$rep {
            new_vec.push($e);
        }
        new_vec
    }};
}

#[cfg(test)]
mod vecd_test {

    #[test]
    fn empty_vec() {
        let test_vec: Vec<()> = my_vec![];
        assert!(test_vec.is_empty());
    }

    #[test]
    fn singleton_vec() {
        let test_vec: Vec<_> = my_vec![0];
        assert_eq!(test_vec.len(), 1);
        assert_eq!(test_vec[0], 0);
    }

    #[test]
    fn multi_eles_vec() {
        let test_vec: Vec<_> = my_vec![2, 5, 8];
        assert_eq!(test_vec, vec![2, 5, 8]);
    }

    #[test]
    fn multi_eles_vec_trailing_comma() {
        let test_vec: Vec<_> = my_vec![1, 4, 7,];
        assert_eq!(test_vec, vec![1, 4, 7,]);
    }

    #[test]
    fn rep_ele_vec() {
        let test_vec: Vec<_> = my_vec![1;3];
        assert_eq!(test_vec, vec![1;3]);
    }    
}

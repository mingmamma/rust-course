macro_rules! vecd {
    () => {
        Vec::new()
    };
    ($e:expr) => {{
        let mut new_vec = Vec::new();
        new_vec.push($e);
        new_vec
    }};
    ($($e:expr),+$(,)?) => {{
        let mut new_vec = Vec::new();
        $(new_vec.push($e);)+
        new_vec
    }}
}

#[cfg(test)]
mod vecd_test {

    #[test]
    fn empty_vec() {
        let test_vec: Vec<()> = vecd![];
        assert!(test_vec.is_empty());
    }

    #[test]
    fn singleton_vec() {
        let test_vec: Vec<_> = vecd![0];
        assert_eq!(test_vec.len(), 1);
        assert_eq!(test_vec[0], 0);
    }

    #[test]
    fn multi_entry_vec() {
        let test_vec: Vec<_> = vecd![2, 5, 8];
        assert_eq!(test_vec, vec![2, 5, 8]);
    }
}

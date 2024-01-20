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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macro() {
        let test_vec = vec!(1,2,3);
        println!("{:?}", test_vec)
    }
}  
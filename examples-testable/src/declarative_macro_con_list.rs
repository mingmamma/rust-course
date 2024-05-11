enum List {
    Nil,
    Con(i32, Box<List>),
}

macro_rules! create_list {
    () => {
        {
            List::Nil
        }
    };
    ($head:literal$(, $other_ele: literal)*) => {
        {   
            let tail = create_list!($($other_ele),*);
            List::Con($head, Box::new(tail))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_con_list() {
        let test_list = List::Con(3, Box::new(List::Con(2, Box::new(List::Con(1, Box::new(List::Nil))))));

        let test_list_2 = create_list!(3, 2, 1);

        assert!(true);
    }
}
// doc.rust-lang.org/book/ch15-02-deref.html

use std::ops::Deref;

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn return_str(s: &str) -> &str {
    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_MyBox_deref() {
        let val = 5;
        let boxed_val = MyBox::new(val);

        assert_eq!(val, 5);
        assert_eq!(*boxed_val, 5);
    }

    #[test]
    fn test_return_str_from_my_box_with_deref_coer(){
        let boxed_str = MyBox::new(String::from("rust"));
        let deref_str = return_str(&boxed_str);

        assert_eq!(deref_str, "rust");
    }

    #[test]
    fn test_return_str_from_my_box_with_deref_coer_2(){
        let boxed_str = MyBox::new("rust");
        let deref_str = return_str(&boxed_str);

        assert_eq!(deref_str, "rust");
    }
}
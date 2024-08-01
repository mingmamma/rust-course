struct Group<T> {
    remainder_vec: Vec<T>,
}

impl<T> Group<T> {
    fn new(input_vec: Vec<T>) -> Self {
        Self {
            remainder_vec: input_vec,
        }
    }
}

impl<T: PartialEq> Iterator for Group<T> {

    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // if the backing remainder Vec is empty, the iterator is finished
        if self.remainder_vec.is_empty() {
            return None;
        }

        // otherwise, calculate the proper sub Vec to return as a Some variant as an yielded item of the iterator

        // determine up to which element in the remainder_vec forms a sub Vec of identical elements with an incremental cursor
        let mut cursor = 1;
        for element in &self.remainder_vec[1..] {
            // the application of binary opration `==` involves generic operant values of type &T
            // which is permissible if the generic type T implements trait bound std::cmp::PartialEq
            // and by the extension implementation also available on the related reference types &T/&mut T
            if element == &self.remainder_vec[0] {
                cursor += 1;
            } else {
                break;
            }
        }

        // remove the identified sub Vec from the remainder Vec in place and return as a yielded item of the iterator
        let items = self.remainder_vec.drain(0..cursor).collect();
        Some(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_how_group_iterator_works() {
        // given an input Vec, e.g. vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3]
        // the group iterator transformed from the input is an iterator whose
        // individual items to be yielded are sub Vecs of the input Vec s.t.
        // elements in each yielded sub Vec are the same, visually illustrated:
        // input:          vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3]
        // group transformed: | 1| 2, 2| 1, 1| 2, 2| 3| 4, 4| 3|

        let test_vec = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
        assert_eq!(
            Group::new(test_vec).collect::<Vec<Vec<i32>>>(),            
            vec![
                vec![1],
                vec![2, 2],
                vec![1, 1],
                vec![2, 2],
                vec![3],
                vec![4, 4],
                vec![3],
            ]
        ); 
    }

   
}


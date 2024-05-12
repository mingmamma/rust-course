pub struct AverageCollection {
    inner_vec: Vec<i32>,
    avg: f64,
}

// a Vec backed collection (of INTs) with a ready public interface for querying its average value (in a FLOAT)
impl AverageCollection {
    pub fn new(input_vec: Vec<i32>) -> Self {
        let mut result = AverageCollection {
            inner_vec: input_vec,
            avg: f64::default(), /* 0.0 */
        };
        result.update_avg();
        result
    }

    pub fn add(&mut self, value: i32) {
        self.inner_vec.push(value);
        self.update_avg();
    }

    pub fn pop(&mut self) -> Option<i32> {
        let pop_result: Option<i32> = self.inner_vec.pop();

        pop_result.map(|popped_val: i32| -> i32 {
            self.update_avg();
            popped_val
        })
    }

    pub fn get_avg(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        if self.inner_vec.is_empty() {
            self.avg = f64::default();
        } else {
            let vec_sum: i32 = self.inner_vec.iter().sum();
            self.avg = vec_sum as f64 / self.inner_vec.len() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut test_col = AverageCollection::new(vec![1, 2, 3]);
        test_col.add(4);
        assert_eq!(test_col.get_avg(), 2.5);
    }

    #[test]
    fn test_pop() {
        let mut test_col = AverageCollection::new(vec![1, 2]);
        test_col.pop();
        assert_eq!(test_col.get_avg(), 1 as f64);
        test_col.pop();
        assert_eq!(test_col.get_avg(), 0.0);
    }
}

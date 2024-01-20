pub struct AverageCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AverageCollection {
    pub fn new(input_list: Vec<i32>) -> Self {
        let mut result = AverageCollection {
            list: input_list,
            avg: 0.0,
        };
        result.update_avg();
        result        
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_avg();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_avg();
                Some(value)
            }
            None => None,
        }
    }

    pub fn get_avg(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.avg = sum as f64 / self.list.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut avg_coll = AverageCollection::new(vec![1,2,3]);
        avg_coll.add(4);
        assert_eq!(avg_coll.get_avg(), 2.5);
    }
}
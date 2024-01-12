pub struct AverageCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AverageCollection {
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
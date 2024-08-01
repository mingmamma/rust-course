struct FibIterator {
    memo: [u64; 2],
}

impl FibIterator {
    fn new() -> FibIterator {
        FibIterator {
            memo: [0, 1],
        }
    }
}

impl Iterator for FibIterator {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
            let next_val = self.memo[0] + self.memo[1];
            self.memo[0] = self.memo[1];
            self.memo[1] = next_val;
            Some(next_val)
        }
    }

mod test {
    use super::*;

    // a straighforward recursive implementation of to calculate the fib
    // give a position by sticking to the basic definition to help the
    // test of the correctness of other implementations
    fn basic_fib(n: usize) -> u64 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            basic_fib(n-1) + basic_fib(n-2)
        }
    }

    #[test]
    fn test_fib_reccur() {
        let test_fib_iter = FibIterator::new();

        for (idx, gened_fib) in test_fib_iter.take(10).enumerate() {
            assert_eq!(basic_fib(dbg!(idx+2)), gened_fib);
        }
    } 
}
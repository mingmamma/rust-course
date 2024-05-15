struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter { count: 0, length }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    for number in Counter::new(10) {
        println!("{}", number);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);

    let vs = vec![1, 2, 3];

    // Diff of the expectation of iter() and into_iter() methods
    // clearly stated: c.f. https://doc.rust-lang.org/std/iter/index.html#the-three-forms-of-iteration

    // note that the iter() method is applied on the slice
    // of the vector instead of the vector itself s.t. the
    // matched variable is a reference type
    for v in vs.iter() {
        ()
    }

    // By contrast, the into_iter() method is applied on the
    // vector itself and the matched variable is of owned type
    for v in vs.into_iter() {
        ()
    }

    let vs2 = vec![1, 2, 3];
    // Desugaring the for loop of a collection
    // c.f. https://doc.rust-lang.org/std/iter/index.html#for-loops-and-intoiterator
    let f = |x: i32| {
        println!("{}", x);
    };
    // let mut iter = vs.into_iter();
    let _ = match vs2.into_iter() {
        mut iter => loop {
            let next;
            match iter.next() {
                None => break,
                Some(val) => next = val,
            }
            let x = next;
            f(x);
        },
    };
}

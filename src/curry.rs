fn main() {
    let add = |a: i32, b: i32| {a+b};
    let add_1_2 = add(1, 2);
    assert_eq!(add_1_2, 3);

    let add_curried  = move |a: i32| move |b: i32| {a+b};
    let add_1_curried = add_curried(1);
    assert_eq!(add_1_curried(2), 3);
    assert_eq!(add_1_curried(9), 10);

    let is_between = move |min: i32| move |max: i32| move |item: &i32| {min < *item && *item < max};

    // let filter_between = move |min: i32| move |max: i32| move |vec_ref: &Vec<i32>| {
    //     // Note that the iter() method produces each next element as a immutable reference to the element
    //     vec_ref.iter().filter_map(|i| if is_between(min)(max)(i) {Some(*i)} else { None })
    // };
}


fn main() {
    let add = |a: i32, b: i32| a+b;
    let add_1_2 = add(1, 2);
    assert_eq!(add_1_2, 3);

    let add_curried = move |a:i32| move |b:i32| a+b;
    let add_1_curried = add_curried(1);
    assert_eq!(add_1_curried(2), 3);
    assert_eq!(add_1_curried(9), 10);

    let add_1_curried_3 = |a:i32| move |b:i32| a+b;
}
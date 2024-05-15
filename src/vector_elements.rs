fn add_both_ends<T: Clone>(vec: &mut Vec<T>, ele: T) {
    let ele_clone = ele.clone();
    vec.push(ele);
    vec.insert(0, ele_clone);
}

fn extend_by_vec<T>(vec: &mut Vec<T>, vec2: Vec<T>) {
    vec.extend(vec2)
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]

    add_both_ends(&mut v, 10);
    println!("{:?}", v);

    let even_more_numbers = vec![11, 12];
    extend_by_vec(&mut v, even_more_numbers);
    println!("{:?}", v);
}

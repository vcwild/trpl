fn main() {
    let mut vec = Vec::new();

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 0);
    vec.push(1);
    vec.push(2);

    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3].iter().copied());

    // for x in &vec {
    //     println!("{x}");
    // }
    assert_eq!(vec, [7, 1, 2, 3]);

    vec.append(&mut vec![4, 5, 6]);
    assert_eq!(vec, [7, 1, 2, 3, 4, 5, 6]);
    vec.insert(3, 99);
    assert_eq!(vec, [7, 1, 2, 99, 3, 4, 5, 6]);
}

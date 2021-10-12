fn main() {
    assert_eq!(5, vec![1, 2, 3, 4, 5].len());
    im::vector![1, 2, 3, 4, 5];
    let iv = im::Vector::<i32>::new();
    let t = iv.is_empty();
    println!("{}", t);
}

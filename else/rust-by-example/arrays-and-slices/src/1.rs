fn main() {
    let mut x = [1, 2, 3];
    let a = &mut x[..];
    a[1] = 20;
    println!("{:?}", a);
    println!("{:?}", x);
}

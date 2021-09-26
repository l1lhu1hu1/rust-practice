// fn main() {
//     let pom = 4;
//     println!("pom:{} [before test_fn]", pom);
//     test_fn(pom);
//     println!("pom:{} [after test_fn]", pom);
// }

// // 関数の引数の変数の前にmut
// fn test_fn(mut pom: i32) {
//     println!("pom:{} [enter in test_fn]", pom);
//     pom = 13;
//     println!("pom:{} [changed in test_fn]", pom);
// }

fn main() {
    let mut pom = 4;
    println!("pom:{} [before test_fn]", pom);
    test_fn(&mut pom);
    println!("pom:{} [after test_fn]", pom);
}

// 関数の引数の型の前に&mut
fn test_fn(pom: &mut i32) {
    println!("pom:{} [enter in test_fn]", pom);
    *pom = 13;
    println!("pom:{} [changed in test_fn]", pom);
}

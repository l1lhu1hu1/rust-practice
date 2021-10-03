// Tに型の制限をかけている
// PartialOrd traitとCopy traitを満たすような型
fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    // TODO resultが何故再代入可能なのか
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(&char_list);
    println!("The largest char is {}", result);
}

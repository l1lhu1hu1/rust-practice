fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // Tがなりうる全ての可能性のある型に対して動作しない
        // Tは値が順序つけ可能な型のみしかダメ
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn get_largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn get_largest_char(list: &[char]) -> char {
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
    let result = get_largest_i32(&number_list);
    println!("The largest number is {}", result);

    // TODO resultが何故再代入可能なのか
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest_char(&char_list);
    println!("The largest char is {}", result);
}

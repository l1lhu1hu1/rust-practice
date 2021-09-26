fn first_word_end_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    // word will get value 5
    let word = first_word_end_index(&s);
    println!("{}", word);
    s = String::from("good bye");
    println!("{}", word);
    s.clear();
    println!("{}", word);
    // sがメモリから解放された後に、sのword番目を参照しようとするエラーになる
    // wordはsが解放されてもスコープから出るまで残り続ける
}

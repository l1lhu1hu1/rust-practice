fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn main() {
    let mut s = String::from("hello world");
    // word will get value 5
    let word = first_word(&s);
    // あるものに対して、immutable referenceがあるときに、そのものに対するmutable referenceは存在できない
    // clearではmutable referenceが必要になる
    s.clear();
    println!("{}", word);
    // sがメモリから解放された後に、sのword番目を参照しようとするエラーになる
    // wordはsが解放されてもスコープから出るまで残り続ける
}

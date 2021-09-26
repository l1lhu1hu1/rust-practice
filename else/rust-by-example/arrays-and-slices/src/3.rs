// shared_rrrでは引数で受け取った値のコピーをインクリメントする
fn shared_rrr(mut s: i32) {
    s = s + 1;
    println!("s: {}", s);
}

// mutable_rrrでは引数で受け取った値をインクリメントする
fn mutable_rrr(s: i32) {
    s = s + 1;
}

fn main() {
    let a = 20;
    shared_rrr(a);
    mutable_rrr(a);
    println!("a: {}", a);
}

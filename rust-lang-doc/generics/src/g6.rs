struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// Uをselfとして受け取って、Tをもう一つの引数として受け取る
// U, Tはいずれもジェネリック型
// 特定のstructの実装をするわけではない
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
    // null.double_drop(empty);
    // empty;
    // null;
}

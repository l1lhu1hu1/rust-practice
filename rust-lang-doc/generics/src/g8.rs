struct Subject<T> {
    eng: T,
    math: T,
}

trait Random<T> {
    fn bye() {
        println!("bye");
    }
    fn hello(t: T) -> T {
        t
    }
    fn ttt(&self) {}
}

impl<T, U> Random<T> for Subject<U> {
    fn ttt(&self) {
        println!("ttttt");
    }
}

fn main() {
    let s = Subject {
        math: "32",
        eng: "55",
    };
    // 多分こういうimplとtraitとstructの使い方はしない
    // 一旦スキップする
    // s.ttt();
}

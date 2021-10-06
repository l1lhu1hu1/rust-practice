struct Subject<T> {
    math: T,
    eng: T,
}

struct Empty;

trait Point<T> {
    // functionやmethodをこのようにtraitに書いた場合は、
    // Point traitを実装する型(structとか)は関数を実装しなければならない
    fn double_score(self, _: T);
    fn print_hello(self);
}

// 全ての型(U)にPoint Traitを実装する
impl<T, U> Point<T> for U {
    fn double_score(self, _: T) {
        println!("double score called");
    }

    fn print_hello(self) {
        println!("hello hello hello");
    }
}

trait Bye {
    fn say_bye() {
        println!("bye bye bye")
    }
}

impl<T> Bye for Subject<T> {}

fn main() {
    let subject = Subject { math: 45, eng: 20 };
    println!("{}", subject.math);
    Subject::<i32>::say_bye();
    subject.double_score(2);
    // subject.print_hello::<i32>();
    // subject.print_hello::<i32>();
    // Subject::<i32>::print_hello::<i32>();
    // <Subject<i32> as Trait>::Point::<i32>::print_hello();
    // let empty = Empty;
    // empty.double_score(0);
}

use crate::List::*;
// 以下のように一々 List::variantってするのがめんどくさい
// useを使うことで、このプログラム内ではNilだけ、Consだけで使えるようになる
// List::Nil
// List::Cons(_, ref tail) => 1 + tail.len()

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, elem: u32) -> List {
        // TODO Box以外でもこれは実装できないのかな?
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            // 最後に追加された値、tail(Box型)
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    // TODO list.pop(最初の要素)を実装する
    // TODO list.pop(最後の要素)を実装する

    // 暗黙的な参照外しが発生するので、*selfでもselfでも大丈夫
    // &Self.fieldができるのも同じ理由
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn print(&self) {
        println!("hello, {}", self.x);
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// useを使わないで書いた場合の例をLangで示している
enum Lang {
    Ja,
    En,
    Else,
}

impl Lang {
    fn new(s: &str) -> Self {
        return match s {
            "en" => Lang::En,
            "ja" => Lang::Ja,
            _ => Lang::Else,
        };
    }

    fn get_lang_code(&self) -> i32 {
        match self {
            Lang::Ja => 0,
            Lang::En => 21,
            Lang::Else => -20,
        }
    }
}

fn main() {
    let mut list = List::new();
    // list.prependするときは所有権を渡している
    // 戻り値でlistの所有権が返ってくる
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // lenとstringifyにはborrowしている
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
    println!("##################################");
    let p = Point::new(32, 64);
    p.print();
    println!("##################################");
    let a = 123;
    let b = &a;
    // どっちとも123になる
    println!("b: {}", *b);
    println!("&b: {}", &b);
    println!("##################################");
    let h = Lang::new("elsesdfda");
    let n = h.get_lang_code();
    println!("n: {}", n);
}

use crate::List::*;
// これでも可
// use crate::List::{Cons, Nil};
// 以下のように一々 List::variantってするのがめんどくさい
// useを使うことで、このプログラム内ではNilだけ、Consだけで使えるようになる

enum List {
    Cons(u32, Box<List>),
    Nil,
}

fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}

impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 再帰的にlenは呼ばれている
    // Cons(val1, Cons(val2, Cons(val3, Cons(val4, Cons(...)))))
    fn len(&self) -> u32 {
        println!("called");
        match *self {
            Cons(head, ref tail) => {
                println!("head: {}", head);
                1 + tail.len()
            }
            Nil => 0,
        }
    }

    // opをdepthに変える
    fn random(&self, op: i32) -> Box<List> {
        if op == 1 {
            match self {
                Cons(_, tail) => tail,
                Nil => &Box::new(Nil),
            };
        }
        Box::new(Nil)
    }

    fn pop_front(self: Self) -> Box<List> {
        match self {
            Cons(_, tail) => tail,
            Nil => Box::new(Nil),
        }
    }
    // TODO fn pop_back() -> List {}

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
    println!("################pop start##################");
    println!("{}", list.stringify());
    let list = list.pop_front();
    println!("{}", list.stringify());
    let list = list.pop_front();
    println!("{}", list.stringify());
    let list = list.pop_front();
    println!("{}", list.stringify());
    println!("################pop end##################");
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

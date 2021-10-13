use crate::List::*;
// これでも可
// use crate::List::{Cons, Nil};
// 以下のように一々 List::variantってするのがめんどくさい
// useを使うことで、このプログラム内ではNilだけ、Consだけで使えるようになる

enum List {
    Cons(u32, Box<List>),
    Nil,
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
        match *self {
            Cons(head, ref tail) => {
                println!("head: {}", head);
                1 + tail.len()
            }
            Nil => 0,
        }
    }

    fn pop_front(self: Self) -> List {
        match self {
            // boxの中身をコピーしたい(List型の値が欲しい)ので*tailにしている
            Cons(_, tail) => *tail,
            Nil => Nil,
        }
    }

    fn pop_back(self: Self) -> List {
        let mut v: Vec<u32> = Vec::new();
        self.get_tail_num(&mut v);
        v.remove(v.len() - 1);
        v.reverse();
        let mut list = List::new();
        for a in &v {
            list = list.prepend(*a);
        }
        list
    }

    // Nilの一つ手前の値を削除する
    fn get_tail_num(self: Self, v: &mut Vec<u32>) {
        match self {
            Cons(head, tail) => {
                v.push(head);
                tail.get_tail_num(v);
            }
            Nil => {}
        }
    }

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
    println!("################pop front start##################");
    let mut list = List::new();
    // list.prependするときは所有権を渡している
    // 戻り値でlistの所有権が返ってくる
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    // lenとstringifyにはborrowしている
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
    let list = list.pop_front();
    let list = list.pop_front();
    println!("{}", list.stringify());
    println!("################pop front end##################");
    println!("################pop back start##################");
    let mut l2 = List::new();
    l2 = l2.prepend(1);
    l2 = l2.prepend(2);
    l2 = l2.prepend(3);
    l2 = l2.pop_back();
    l2 = l2.pop_back();
    println!("after popback: {}", l2.stringify());
    println!("################pop back end##################");
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

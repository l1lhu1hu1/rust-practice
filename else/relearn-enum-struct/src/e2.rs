enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 上のMessage enumを構造体で表した場合、以下のようになる
// QuitMessage, MoveMessage, WriteMessage, ChangeColorMessageの
// どれを引数とってもいいような関数の定義は難しい
// enumを使えば, impl Messageの中にメソッドを定義するだけで実現できる
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// enumにメソッドを定義することもできる
impl Message {
    fn call(&self) {
        println!("hello");
        match self {
            Message::Write(value) => println!("write, {}", value),
            _ => println!("else"),
        }
    }
}

fn main() {
    let m = Message::Write("llllllllllllllllllllllllll".to_string());
    m.call();
}

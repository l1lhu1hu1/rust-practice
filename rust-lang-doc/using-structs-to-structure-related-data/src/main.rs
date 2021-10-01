// RectangleにDebugという関数をどこかから引き出してきて与えているイメージ
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    // 以下を実行するとエラーが出てきて、{:?}にしたらいけるかも?って出てくる
    // println!("Hello, world!, {}", rec1);
    // 以下を実行するとエラーが出てきて、println!使うにはdebugって関数が型に実装されていないダメってでてくる
    // println!("Hello, world!, {:?}", rec1);
    println!("rectangle is {:#?}", rec1);
}

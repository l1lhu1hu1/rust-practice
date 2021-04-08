use std::thread;
fn main() {
    // spawnにはクロージャーを渡す。引数は|variable|のようにして与える。今回は引数がないので||
    let handle = thread::spawn(|| {
        println!("Hello, world!");
    });
    // プログラムがHello, worldの表示よりも先に終わってしまう可能性があるので、handle.join()している
    // handle.join()によってスレッドの終了を待つことができる
    dbg!(handle.join());
}

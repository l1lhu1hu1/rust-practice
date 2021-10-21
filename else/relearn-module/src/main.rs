// libに現在はmoduleを書いているので、relearn_moduleクレート指定しないといけない
// もしmain.rsもlib.rsの2つが存在している場合には、両方ともパッケージと同じ名前を持つクレートになる
use relearn_module::hello::*;
use relearn_module::sayonara::*;

// libに書いてある内容をmainに全部書く場合の例
// mod bye;
// pub mod hello;
// pub mod sayonara;
// use self::hello::*;
// use self::sayonara::*;

fn main() {
    println!("Hello, world!");
    say_hello();
    say_sayonara();
}

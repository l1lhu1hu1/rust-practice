use crate::bye;
// 下のコードはエラーになる
// use relearn_module::bye;
pub fn say_sayonara() {
    println!("sayonara called");
    bye::say_bye();
    bye::main();
}

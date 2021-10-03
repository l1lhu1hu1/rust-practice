fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // ここでrを参照しようと思ってもxがスコープを抜けてて、xが解放されてしまっている
    println!("r: {}", r);
}

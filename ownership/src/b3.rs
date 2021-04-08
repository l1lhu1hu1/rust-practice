fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        // 2回目の可変な参照渡し(これによってエラーになる)
        let z = &mut x;
        dbg!(y);
        dbg!(z);
    }
    {
        let y = &x;
        // mutableとimmutableの参照は共存できないのでエラーになる
        let z = &mut x;
        dbg!(y);
        dbg!(z);
    }
}

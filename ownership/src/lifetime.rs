fn main() {
    // {
    //     let mut x = 5;
    //     let y = &x;
    //     // immutableとmutableな参照は共存できない(yは後で登場する.ライフタイムが尽きていない)のでエラーになる
    //     let z = &mut x;
    //     dbg!(y);
    //     dbg!(x);
    // }
    {
        let mut x = 5;
        // yはこれ以降で登場しないのでここでライフタイムは尽きる
        let y = &x;
        // ここはエラーにならない(immutableな参照であるyのライフタイムが尽きたため)
        let z = &mut x;
        dbg!(z);
        dbg!(x);
    }
}

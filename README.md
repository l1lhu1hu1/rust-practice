# Rustノート
## Rustの説明
- JavaScriptの作者やMozilla Research社の現Swift開発者が中心になって設計した言語
- 実行速度が早くて、モダンな文法が一通り入っている
- OSからWebアプリまで幅広く開発可能
- 安全性が強力に担保されている
- CやC++の次にパフォーマンスが良い

### Rustが速い理由
- (OSに合わせた)機械語に直接コンパイルされる
- ガーベージコレクションを持たない
- ゼロコスト抽象化を追求している

## Rustと多言語の比較
### JavaとPython
1. コードがインタープレタに渡り、仮想マシン用のコードへと変換される
2. 仮想マシンにて実行しているOSに合わせて機械語へと変換される
3. 仮想マシンから渡ってきた機械語を実行する

#### 疑問点
- どういう単位で実行するのに必要なコンパイラ等のツールがインストールされているのか
- エンジンとはひとまとめになっているものか(コンパイルから実行までできるイメージ)
- LLVMを何故Javaなどの言語でも使用していないのか

### GoとRust
両方とも仮想マシンを持たない。RustではLLVMを用いて環境に合わせた機械語を生成している。Java, Python, Goではガーベージコレクションを利用している。C, C++, Rustではガーベージコレクションを利用していない。CやC++では手動でメモリの確保・解放をすることでメモリ管理を行う。ガーベージコレクションを利用しない方が速いと言われている。早い理由に関してはガーベージコレクションの説明を参照。

## メモリ管理
### ガーベージコレクション
ガーベージコレクションでは、不要なメモリ領域(プログラムが実行中に確保したメモリ領域のうち、今後使う予定のないメモリ領域のうち今後使う予定のない領域)を解放し、利用可能なメモリ領域を増やすという処理を逐一行っている。開発者がメモリ管理を意識せずに開発ができるのは良いことだが、ガーベージコレクションには以下のような問題がある。
- 不要なメモリ領域がいつ開放されるかわからない
- 不要なメモリを開放する瞬間に数ミリから数秒の間、本体の処理を止めてしまう可能性がある。この停止している時間がプログラムが行う処理全体から見ると長い

#### 疑問点
不要なメモリ領域がいつ開放されるかわからない?

### 手動でメモリ管理
ガーベージコレクションで起きるような問題は起こらないが、その代わりに手動で管理する場合は、開発者がメモリを意識して開発を行わないといけない。また人間が手動で管理するのでミスが起こりやすい。具体的には以下のようなミスが起きる。
- メモリ領域の解放忘れ
- 解放している領域を再度解放しようとする(use-after-free)

メモリアクセス時に発生するバグやセキュリティホールなどから保護されている状態のことを*「メモリ安全」*であるという。手動で管理する場合は、メモリアクセス時に発生するバグから保護されていないのでメモリ安全ではない。

## 型推論
コンパイル時にコンパイラが型の情報を推論して補完する仕組み。静的型付け言語において、すべての変数に対して型をつけなくても、一部の変数はコンパイラ側で推論してくれるってイメージ。Goで一部の変数に型を書かなくてもいいのはこれのおかげ。動的型付け言語(RubyやJavaScript)では、実行時にしか型に関するエラーは出せない。

- [型推論のしくみ](https://www.klab.com/jp/blog/tech/2015/1047569315.html)
- [人でも分る型推論](https://qiita.com/uint256_t/items/7d8c8feeffc03b388825)

#### 疑問点
もう少し型推論について調べて理解する

## Rustの安全性
スレッド安全かメモリ安全かをコンパイル時にチェックする。ダメな場合はエラーを吐く

#### 疑問点
他の言語ではどうなのか(JavaScript, Java, Python等で)

## Trait
- JavaやGoのインターフェースのような機能
- トレイトとは(未知の型(Self)のための)メソッドの集合
- インターフェースとオブジェクトの紐付けは実行時に行われるが、RustのTraitではコンパイル時に行われる

## Scoping Rules
### self
`&self` is sugar for `self: &Self`, where `Self` is the type of the *caller Object*

## ジェネリクス
プログラミング言語の機能・仕様の一つ。同じプログラムコードで様々なデータ型のデータを処理できるようにする機能。ジェネリクスを用いると、特定のデータ型に決め打ちせずに処理内容を記述できる。ジェネリクスは使われるまで型が決まらないような色々な型の値を受け入れる機能を作る時に使う。

単相化とはコンパイル時に使用されている具体的な型を入れることでジェネリックなコードを特定のコードに変換する過程のこと。Rustのコンパイラはコードの単相化を行う。Rustはジェネリックなコードが呼び出されている箇所全部を見て、ジェネリックなコードが呼び出されている具体的な型のコードを生成する。

### implにおけるジェネリクス
```
// イメージ的にはTは引数
// idの型がT. 外部から型を切り替えられる
struct GenericVal<T> {
    id: T,
}

// GenericValのTがi32であるときのみ中の実装は定義される
impl GenericVal<i32> {
    // fooはTがi32であるときだけ定義されている
    fn foo(&self) {
        println!("integer: {}", self.id);
    }
}

// GenericValのTがcharであるときのみ中の実装は定義される
impl GenericVal<&str> {
    // fooはTがcharであるときだけ定義されている
    fn foo(&self) {
        println!("string: {}", self.id);
    }
}

// if you want to write an impl block that applies for all GenericVal<T> types,
// you must first declare a type parameter on the impl itself
// (otherwise, T would try to look up a type named T)

// GenericValのRがどんなものでも中の実装は定義される
// implの直後のRがなければ、GenericValの後のRはジェネリックな型ではなく
// Rという名前の型を指定することになってしまう
impl<R> GenericVal<R> {
    // hogeはTがどんな型であっても定義されている
    fn hoge(&self) -> &str {
        "hoge called"
    }
}
```

以下のようなコードは、DisplayをTraitとして実装するあらゆる型にToStringを実装している

```
impl<T: Display> ToString for T {
    //省略...
}
```

## トレイト
### 引数としてのトレイト

以下はSummary Traitを実装している型を引数として受け付ける例。イメージ的にはある型を継承している型のみを許す感じ

```
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}
```

以下のようにジェネリクスを利用しても同じことができる

```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### トレイトを実装している型を返す
impl trait構文を戻り値のところで使えば、あるトレイとを実装する何らかの型を返すことができる。戻り値に制限を設けることができる。

```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

## ライフタイム
### 関数とライフタイム
関数呼び出しを行う時に、関数に与える引数の変数(参照)をどのタイミングで解放すればいいかわからない。コンパイラが関数呼び出しの時に、戻り値のライフタイムを推論することも場合によってはできる。できる場合というのは、引数が一つの場合。できない場合には、プログラム上なんとかするしかない。そこで、呼び出す関数に注釈をつける。これによって、注釈がついている引数と戻り値は同じライフタイムであることをコンパイラに伝えられる。2つ以上引数があって、そのそれぞれに注釈がついている場合には、それらのうちライフタイムが短いものが、戻り値のライフタイムとなる。

```
fn main() {
    let z;
    let x = "foo".to_string();
    {
        let y = "bar".to_string();
        // yと引数の1つ目のライフタイムは同じで
        // printlnがある行まで生きられないのでエラーになる
        // 参照元の値よりも参照のライフタイムが長生きするのは許されない
        // z = some_method(&y, &x);

        z = some_method(&x, &y);

        // 以下の場合だと、戻り値のライフタイムはyのライフタイムと同じになる
        // z = hello_method(&x, &y);
    }
    println!("{}", z);
}

// arg2に関してwarningが出るが、コンパイルはできる
// arg1のライフタイムが第一引数のxと同じであることになる
// xと同じライフタイムなので、some_methodの結果はprintlnがある行まで生き続ける
fn some_method<'a>(arg1: &'a String, arg2: &String) -> &'a String {
    arg1
}

// 戻り値はライフタイムが短い方のライフタイムと同じになる
fn hello_method<'a>(arg1: &'a String, arg2: &'a String) -> &'a String {
    arg1
}
```

## マクロ
マクロは、コンパイラがコードの意味を解釈する前に展開され、手で書いたよりも多くのコードを生成する。大きく分けて、Rustのマクロには以下の2種類がある。
- 宣言的マクロ(Declarative macro)
- 手続き的マクロ(Procedural macro)

### 宣言的マクロ
パターンマッチングを行い、マッチしたコードをほかのコードで置き換える。

```
macro_rules! hoge {
    ($x: expr) => {
        println!("{}", $x)
    };
}
```

### 手続き的マクロ
コードを入力として受け取り、そのコードに対して作用し、出力としてコードを生成する。以下のコードはPancakesにHelloMacroを新たに実装することを表したコード。

```
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

## enum
`main.rs`と`e2.rs`を詳しくはみる。

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### option
- nullとはそこに何も値がないことを意味する値
- nullに対して存在しないメソッドの呼び出しを行ったりするとエラーになる
- nullをnullでないかのように扱ってしまうとエラーになる

Option型は標準ライブラリで定義されているenum。Rustではnull(RustではNoneという)になる可能性がある値を保持するには、その値の型をOption<T>にしなければならない。そのような値を使用するときには、値がnullである場合を明示的に処理する必要がある。

T型の処理を行う前に、Option<T>をTに変えるステップを必ず挟む必要がある。これにより今まであったnullの問題を解決できる。これまで、他の言語ではこのステップを挟まないでnullが返ってくるか、想定していた値が返ってくるかがわからないことで、nullに関するエラーは起こっていた。以下のコードでは計算はできない。なぜなら、xはi8型でyはOption型だから。このようにワンステップ入ることで、xとnullになるかもしれない値を足し合わせることはできない

```
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
```

### match
- パターンの表現力とコンパイラが全てのありうるパターンを処理しているかを確認してくれるという事実に由来します。


## 参考・引用
### Rust By Example
- [Rust Book](https://doc.rust-lang.org/book/title-page.html)
- [Rust By Example 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)
- [ジェネリクス Future Architect Blog](https://future-architect.github.io/typescript-guide/generics.html)
- [ジェネリクス IT辞典](https://e-words.jp/w/%E3%82%B8%E3%82%A7%E3%83%8D%E3%83%AA%E3%82%AF%E3%82%B9.html#:~:text=%E3%82%B8%E3%82%A7%E3%83%8D%E3%83%AA%E3%82%AF%E3%82%B9%E3%81%A8%E3%81%AF%E3%80%81%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E,%E3%83%86%E3%83%B3%E3%83%97%E3%83%AC%E3%83%BC%E3%83%88%E3%80%8D%EF%BC%88template%EF%BC%89%E3%81%A8%E3%81%84%E3%81%86%E3%80%82)
- [Rust入門 ジェネリクス](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/8ccf20)
- [Rustの所有権、参照、ライフタイムの初歩](https://numb86-tech.hatenablog.com/entry/2021/05/22/195352)
- [マクロ](https://doc.rust-jp.rs/book-ja/ch19-06-macros.html)

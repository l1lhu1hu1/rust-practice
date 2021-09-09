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

## 参考・引用
### Rust By Example
- [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- [Rust By Example 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)


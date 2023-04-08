# Rustのお勉強

複雑系の計算や可視化などをやることでRustを学ぼうと思います．

## 資料など

- [The Rust Programming Language](https://doc.rust-lang.org/book/): 公式のドキュメント．日本語訳もあるらしい．
- [rust-analyzer](https://rust-analyzer.github.io/): VSCodeでrustを書くときの便利拡張機能

## Rustのインストール

[公式のGet Started](https://www.rust-lang.org/ja/learn/get-started)の通りインストールする:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Hello, world

ひとまずはrustが使えるのを確認

```bash
mkdir hello_world
cd hello_world
```

```rust title="main.rs"
--8<-- "rust_study/hello_world/main.rs"
```

```bash
rustc main.rs
./main
# Hello, world!
```

## Cargoの導入

パッケージマネージャーであるCargoを導入する．公式の通りrustをインストールすれば一緒に入るらしい．プロジェクトを作る場合は

```bash
cargo new hello_cargo --vcs=git
```

とする．`hello_cargo`ディレクトリが作成され，`src`ディレクトリとパッケージ情報の`Cargo.toml`が中に入っている．末尾の`vcs`オプションでgitを指定して作成するとgitignoreファイルも一緒に生成される．`Cargo.toml`の中身はこうだった：

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

`src/main.ts`は先ほどのHello, Worldプログラムと同じものになっていた．これをビルドして実行する．

```bash
cargo build
./target/debug/hello_cargo
# Hello, world!
```

ビルドすると`target`下にコンパイルされた実行ファイルと，`Cargo.lock`が生成（または更新）される．lockファイルは外部パッケージ（＝クレートと呼ぶらしい）の依存関係を記録したファイルである．デフォルトではデバック用にコンパイルされるが，最適化されたリリース用バイナリを作るなら`cargo build --release`とすればよい．

`cargo run`はビルドして実行ファイルを叩いてくれる．またソースがコンパイル可能かどうかだけを判定する`cargo check`がある．checkはビルドよりも早いので実装しているときに便利らしい．

## 超基本的な文法

Rustの基本的な文法の一部をまとめて見る．所有権やStructとかはとりあえずおいておく．

```rust title="basic_functionality/src/main.rs"
--8<-- "rust_study/basic_functionality/src/main.rs"
```

## 所有権

モダンなコンピュータのメモリにはヒープとスタックという2種類の領域がある．ヒープは実行時に動的な領域確保ができる代わりにアクセスが遅く，スタックは事前に大きさが分かっている必要がある代わりにアクセスが早い．Rustも含めプログラミング言語にはこれらの領域を使い分ける機能がある．たとえばGabage Collector(GC)はヒープに確保されたメモリが使われなくなったときに自動的に領域解放を行う．しかし解放のタイミングを判定するのは原理的に難しい．

RustはGabage Collectorを持たず，Cのように明示的なメモリ解放も要求しない．その代わりRustは所有権と呼ばれるルールでメモリを管理している:

1. それぞれの値は`owner`をもつ．
2. `owner`はある時刻において1つしかない．
3. `owner`がスコープから外れたときに値＝メモリは破棄される．

たとえばデータ型`String`は可変長な文字列であり，ヒープ領域に実体＝文字データのバイト列が保存され，スタック領域に実体の場所（ポインタ）や長さの情報が格納されている．この`String`型のデータを以下のようにコピー(厳密には浅いコピー)した場合，`s1`と`s2`は同じポインタを持つ．このように複数のポインタが同じヒープ領域をさしている場合，ルール3のブロック終了時での解放が衝突してしまう(double free error)ように思える．しかしルール2によりコピーや関数での引き渡しの際に所有権が移るためdouble freeは回避できる．

```rust
fn foo(s: String) {
    println!("{}", s);
}

fn bar(s: String) {
    s
}

fn main() {
    let s1 = String::from("foo");
    let s2 = s1; // ここでs1の所有権(owner)はs2に移行した
    println1!("{}", s1); // s1は所有権を持たないためコンパイルエラーになる
    let s3 = bar(s2) // mainのs2 -> barのs -> mainのs3
    foo(s3); // ここで所有権は`foo`に移り，fooの終了と共に解放される
    println1!("{}", s2); // s2は所有権を持たないためコンパイルエラーになる
}
```

Rustでは基本的に浅いコピーが使われる．ただし`Copy` Traitが実装されている`u32`などの変数はデフォルトで深いコピーが行われるため，上記の例のような所有権移行の問題はおきない．具体的には整数型, boolean型，浮動小数点型，文字型`char`, Tupleでは`Copy`が実装されている．

## 所有権の参照と借用

変数の所有権の参照を関数の引数として渡すことができる．これを参照の借用(reference borrowing)と呼ぶ．

```rust
--8<-- "rust_study/ownership/src/main.rs"
```

値の同一性を保証するため，変更可能な参照(mutable reference)は複数行うことができない．またimmutable referenceがすでに貸し出されているときに追加でmutable referenceを貸し出すこともできない．

参照を返す関数を実装する場合に実体がない参照（dangling pointer）を返す関数はコンパイルできない:

```rust
fn main() {
    let x = foo();
}

fn foo() -> &String {
    let s = String::from("yead");
    &s // コンパイルエラー
} // ここでsが解放される
```
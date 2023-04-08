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

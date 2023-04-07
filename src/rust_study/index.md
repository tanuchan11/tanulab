# Rustのお勉強

複雑系の計算や可視化などをRustでやることでRustを学ぼうと思います．

## 資料など

- [The Rust Programming Language](https://doc.rust-lang.org/book/): 公式のドキュメント．日本語訳もあるらしい．

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
# proconio_enum_query

![Rust](https://github.com/tyu-ru/proconio_enum_query/workflows/Rust/badge.svg)

[proconio](https://crates.io/crates/proconio)で列挙型を受け取れるようにするattributeを追加するクレートです。

現在、**ジェネリック列挙型**や**Vecの入力**には**対応してない**ので注意です

```rust
#[macro_use]
extern crate proconio_enum_query as _;

#[proconio_enum_query]
enum Query {
    A(i64),
    B(char),
    C,
    D { x: i64, y: i64, s: String },
}

proconio::input! {
    n: usize,
    q: [Query; n],
}
```

[`cargo-equip`](https://github.com/qryxip/cargo-equip)による展開にも対応しています。

参考: [競技プログラミングにprocedural macroを持ち込む](https://qiita.com/qryxip/items/1b4716b1357c89adeaae)

## Description

列挙型に`#[proconio_enum_query::proconio_enum_query]`を付与することで`proconio::source::Readable`が実装されます。

`#[proconio_enum_query]`が付与された列挙型が`input!`で入力されるときは次の順で処理されます。

1. クエリの種類を示す番号として入力の先頭の整数1つが`isize`型として入力処理される
1. クエリ番号が列挙型のヴァリアントの宣言順に対応して、どのヴァリアントとして入力されるか決定される(デフォルトでは**1-indexed**)
1. 選択されたヴァリアントが入力処理される

クエリ番号の開始インデックスを変更したい場合は`#[proconio_enum_query(start_index=0)]`の様に指定できます。
指定できる範囲は`isize`の範囲です。

クエリ番号に該当するヴァリアントが存在しない場合は`panic!`します。

ヴァリアントの種類はユニット型、タプル型、構造体型に対応しています。

各ヴァリアントのフィールド内の型は`proconio::source::Readable`を実装している必要があります。
列挙型は内部の各型を`<T as Readable>::Output`に置き換えたものに変更されます。
([proconio_derive::derive_readable](https://docs.rs/proconio-derive/0.2.1/proconio_derive/attr.derive_readable.html)と同様の挙動です。例えば`Usize1`は`usize`に置き換わります。`input!`の際は`Usize1`として扱われます。)

```rust
#[proconio_enum_query]
enum Query {
    A(i64),        // クエリ番号1
    B(Usize1),     // クエリ番号2
    C(i64, Bytes), // クエリ番号3
}

↓

enum Query {
    A(i64),
    B(usize),
    C(i64, Vec<u8>),
}
```

(実際はもっと汚い感じに置換されるけどそこは目を瞑っていただきたいです)

次のような先頭にクエリの種類を示す番号があり、その番号によって入力形式を切り替えるといった形式の時に便利...かもしれないです。

### stdin

```txt
4 // クエリ数
1 12
2 c
3
4 12 34 hello
```

### Before

```rust
input! {
    q: usize,
}
for _ in 0..q {
    input! { ty: usize }
    if ty == 1 {
        input! { x: Usize1 }
        ...
    } else if ty == 2 {
        input! { c: char }
        ...
```

### After

```rust
#[proconio_enum_query]
enum Query {
    A(Usize1),
    B(char),
    C,
    D { x: i64, y: i64, s: String },
}

input! {
    q: usize,
    query: [Query; q],
}
```

この様な入力形式をもつ問題

- [ABC189-E](https://atcoder.jp/contests/abc189/tasks/abc189_e)
- [ABC157-E](https://atcoder.jp/contests/abc157/tasks/abc157_e)

# ChangeLog

- v0.1.2
  - 'cargo-equip'でのバンドル時にpanic!する問題を修正
- v0.1.1
  - `cargo-equip`が`watt`を要求しなくなったのでそれに追従
    - これによってPythonなどの要求が消えた

## TODO

- ヴァリアントごとのクエリ番号の指定
- ジェネリック列挙型の対応
- Vecを含むクエリ

## Note

`cargo-equip`による展開に対応するためのソースコードは<https://github.com/qryxip/competitive-programming-library>の`proc-macros/fastout`の当該部分を一部改変したものを使用しています。

## Author

tyu-ru <tyuru.tw.cpp@gmail.com>

## License

Licensed under the [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/deed).

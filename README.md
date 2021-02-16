# proconio_enum_query

[proconio](https://crates.io/crates/proconio)で列挙型を受け取れるようにするattributeを追加するクレートです

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

[`cargo-equip`](https://github.com/qryxip/cargo-equip)による展開にも対応しています

参考: [競技プログラミングにprocedural macroを持ち込む](https://qiita.com/qryxip/items/1b4716b1357c89adeaae)

## 機能

列挙型に`#[proconio_enum_query::proconio_enum_query]`を付与することで`proconio::source::Readable`が実装されます

入力の先頭の番号が列挙型のバリアントの宣言順に対応します。これは1-indexedです

各バリアントのフィールド内の型は`proconio::source::Readable`を実装している必要があります。列挙型は内部の各型を`<T as Readable>::Output`に置き換えたものに変更されます。(例えば`Usize1`は`usize`に置き換わります。`input!`の際は`Usize1`として扱われます。)

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

次のような先頭にクエリの種類を示す番号があり、その番号によって入力形式を切り替えるといった形式の時に便利...かもしれないです

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

## 実装されていない機能

- 0-indexedなクエリ番号への対応及びクエリ番号の指定
- **ジェネリック列挙型の対応**
- Vecを含むクエリ

## Author

tyu-ru <tyuru.tw.cpp@gmail.com>

## License

TODO

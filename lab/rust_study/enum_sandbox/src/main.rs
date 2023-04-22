
// enumの定義
#[derive(Debug)]
enum E {
    Foo, 
    Bar
}
// enumに関数を定義する
impl E {
    fn foo(&self) {
        println!("hey");
    }
}

// enumを引数とする関数
fn example(x: E) {}

struct Oh {}

// 値を持つenum
enum C {
    Hoge(String),
    Baz(i32),
    Yes(Oh), // Enumはstructも持てる
}

fn take_option(x: Option<i32>) -> Option<i32> {
    // matchはxが取りうる値の可能性を漏れなくarmで表現しなければならない
    // たとえばこの例では`None => None`がないとコンパイルエラーになる
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn take_option2(x: Option<i32>) -> Option<i32> {
    // 定義したarmすべてに引っかからない場合の挙動を`_ =>`を使って表現できる
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}

fn main() {
    let x = E::Foo;
    println!("{:?}", x);
    x.foo();

    example(E::Bar);

    let y = C::Baz(1234);
    let a = C::Yes(Oh {});

    // optionalな変数．sは5またはNoneである．
    let mut s: Option<i32> = Some(5);
    println!("{:?}", take_option(s));
    println!("{:?}", take_option2(None));

    // 以下のmatchはif letを使って書き換えられる．
    // match s {
    //     Some(i) => print!("{}", i + 1),
    //     _ => (),
    // }
    if let Some(i) = s { println!("{}", i + 1) }
    s = None;
    if let Some(i) = s { println!("{}", i + 1) } // 何も表示されない
    if let Some(i) = s {
        println!("{}", i)
    } else {
        println!("s is None") // matchにおける`_`のcaseはelseで書ける
    }
}

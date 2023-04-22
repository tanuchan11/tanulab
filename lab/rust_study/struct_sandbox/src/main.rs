#[derive(Debug)] //println! macro
struct Foo {
    x: bool,
    y: String,
    z: i32
}
// tuple struct
struct Color(u8, u8, u8);

// メソッド
impl Color {
    fn sum(&self) -> u8 {
        self.0 + self.1 + self.2
    }

    // 値を変更するmethod
    fn double(&mut self) {
        // mutatate Self (=Color)
        self.0 *= 2;
        self.1 *= 2;
        self.2 *= 2;
    }
    // Selfを引数に持たないmethodはassociated functionと呼ばれる．
    // コンストラクタ的に使われる．
    fn white() -> Self {
        Self(255, 255, 255) 
    }
}

// 1つのstructは複数のimplを持つことができる
impl Color {
    fn blak() -> Self {
        Self(0, 0, 0)
    }
}

fn main() {
    // mutable struct
    let mut x = Foo {
        x: true, y: String::from("abcd"), z: 2
    };
    x.y = String::from("hello");
    println!("x = {:?}", x);
    println!("{}", x.y);


    // immutable struct initialized by x
    let y = Foo {
        x: false, ..x
    };
    x.y = String::from("Yes");
    println!("{}", y.y);

    let mut z = Color(0, 1, 2);
    println!("{}, {}", z.0, z.sum());
    z.double();
    println!("{}", z.sum());
    dbg!(&x); // dbg!は所有権を一度借用してstd errorにメッセージをだして所有権を戻す

    let a = Color::white();
    println!("{}", a.0);

}

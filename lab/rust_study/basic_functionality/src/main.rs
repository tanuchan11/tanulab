use std::cmp::Ordering; // Orderingパッケージを使えるようにする

fn main() {
    // これはコメント

    let x = 5; // constなinteger32の変数`x`を初期化
    println!("x = {x}");

    {
        let mut x = 10; // {}のスコープ内でのみxは5ではなく10として振る舞う
        x *= 2;
        println!("x = {x}") // 20
    }
    println!("x = {x}"); // 5

    let mut y; // mutableな変数
    y = 10 + x;
    println!("y = {y}"); // y = 15

    let z = 1.234567; // float64
    println!("z = {z}"); // z = 1.234567

    // Tuple: 要素のデータ型がバラバラでも大丈夫だが固定長（ヒープ）
    let a = (1, "2", 3.1234);
    println!("a[1] = {}", a.1); // a[1] = 2
    
    // Array: 要素のデータ型はすべて同じでなければならないが可変長(スタック)
    let mut b = [1, 2, 3];
    b[0] *= 2;
    println!("b[0] = {}, length = {}", b[0], b.len()); // b[0] = 2, length = 3

    // 関数の呼び出し
    println!("{}", hello(5)); // 😇😇😇😇😇

    control_flow();
}

fn hello(x: usize) -> String{
    // 非負の整数`x`回だけ😇を繰り返した文字列を返す関数

    // Rustの文はStatementとExpressionがある．
    // Statementは値を返さない文であり，たとえば`let x = "foo";`など
    // Expressionは処理結を返す文．下の例では`return "😇".repeat(x)`としても動くが
    // returnがなくても関数の中の末尾のexpressionが返り値になる
    "😇".repeat(x)
}

fn control_flow() {
    // if expression. {}で囲まれたコードブロックはarmと呼ばれる．
    let x = 0;
    if x > 0 {
        println!("x > 0");
    } else if x == 0 {
        println!("x == 0");
    } else {
        println!("x < 0");
    }

    let y = if true {10} else {5};
    println!("{}", y); // 10

    // loop. Pythonでいう`while true`
    // breakで抜けられるがloopにラベルをつけておくと多重ループの制御がしやすくなる
    let mut i = 0;
    'foo: loop {
        loop {
            i += 1;
            if i == 2 { break }
        }
        println!("in foo loop");
        break 'foo
    }
    
    // while
    i = 0;
    while i < 10 {
        i += 1;
    }
    println!("i = {}", i); // 10

    // for
    let a = [1, 2, 3, 4];
    for elem in a {
        i = elem;
        if elem == 3 {break}
    }
    println!("after for: i = {}", i); // afger for: i = 3

    i = 0;
    for j in (1..4).rev() {
        i += j;
    }
    println!("j = {}", i); // i = 6 = 3 + 2 + 1;

    // match
    match i.cmp(&0) { // iを0と比べる
        Ordering::Less => println!("i < 0"),
        Ordering::Equal => println!("i = 0"),
        Ordering::Greater => println!("i > 0")
    }
    
}
fn main() {
    let mut x = String::from("hello");
    println!("{}", foo(&x)); // 5
    bar(&mut x);
    println!("{}", x); // hello, world
    
}

fn foo(s: &String) -> usize {
    s.len() // 変更できない参照(immutable reference)
}

fn bar(s: &mut String) {
    s.push_str(", world"); // 変更可能な参照(mutable reference)
}
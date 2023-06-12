fn main() {
    // list1();
    // list2();
    list3();
}

// 変数のスコープが終わると値は開放される
fn list1() {
    {
        let a = String::from("hello");
    }
    // println!("{}", a)
}

// ムーブしたあとの変数は使えない
fn list2() {
    let a = String::from("hello");
    let b = a;
    println!("{}", b);
    // println!("{}", a)
}

// 参照を使うと所有権は失われない
fn list3() {
    let a = String::from("hello world");
    print_string(&a);
    let b = a;
    // print_string(&a);
}

fn print_string(a: &String) {
    println!("{}", a);
}

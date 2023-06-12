fn main() {
    // list1();
    list2();
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

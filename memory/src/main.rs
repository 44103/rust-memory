fn main() {
    // list1();
    // list2();
    // list3();
    // list5();
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

// Rust だとコンパイルエラーで実行時のエラーを防げる
// fn list5() {
//     let p = get_point();
//     println!("{} {}", p.x, p.y)
// }

struct Point {
    x: i32,
    y: i32,
}

// fn get_point<'a>() -> &'a Point {
//     let p = Point { x: 0, y: 0 };
//     &p
// }

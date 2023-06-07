fn main() {
    list1();
}

// 変数のスコープが終わると値は開放される
fn list1() {
    {
        let a = String::from("aaa");
    }
    // print!("{}", a)
}

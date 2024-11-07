fn main() {
    let s1 = String::from("hi,中国");

    let h = s1.chars().nth(0).unwrap();
    assert_eq!(h, 'h');

    let h1: String = s1.chars().nth(3).unwrap().to_string();
    assert_eq!(h1, "中");

    println!("Success!");
}
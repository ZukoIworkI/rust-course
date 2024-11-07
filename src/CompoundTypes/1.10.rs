fn main() {
    let raw_str = "Escapes don't work here: ? ℝ";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Успіх!");
}
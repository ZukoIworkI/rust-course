
fn main() {
    let mut s = String::from("hello world");

    let letter = first_letter(&s);

    s.clear();

    println!("the first letter is: {}", letter);
}

fn first_letter(s: &str) -> String {
    s[..1].to_string()
}
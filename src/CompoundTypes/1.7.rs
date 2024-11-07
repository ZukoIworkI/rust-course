fn main() {
    let s = "hello, world";
    greetings(s);
}

fn greetings(s: &str) {
    println!("{}", s);
}
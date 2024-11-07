fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    let name0 = names.get(0).unwrap();

    let name1 = names.get(1).unwrap();

    println!("Success!");
}
fn main() {

    let mut list: [i32; 100] = [0; 100];

    list[0] = 1;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
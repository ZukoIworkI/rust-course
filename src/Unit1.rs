#[test]
fn main() {
    let _v: () = ();

    let v = (); // Ожидаемое значение — unit type ()
    assert_eq!(v, implicitly_ret_unit()); // Теперь сравнение корректное

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

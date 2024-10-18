#[test]
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            Some(42)
        }
        _ => {
            never_return_fn()
        }
    }
}
fn never_return_fn() -> ! {
    std::process::exit(1);
}

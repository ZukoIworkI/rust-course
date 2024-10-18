
#[ignore]
#[test]
fn main() {
    never_return();
}

fn never_return() -> ! {
    loop {

    }
}

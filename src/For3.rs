#[test]

fn main() {
    for n in 1..=100 {
        if n == 100 {
            println!("Reached 100, but no panic!");
            break; // or continue if you want to keep looping
        }
    }

    println!("Success!");
}

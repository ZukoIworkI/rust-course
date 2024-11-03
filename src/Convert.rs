#[test]
const WIDTH: usize = 10;
const HEIGHT: usize = 5;

fn main() {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 || x == y || x == WIDTH - y - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

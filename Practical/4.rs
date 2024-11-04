const SIZE: usize = 5;

fn main() {
    let mut output = String::new();

    for i in 0..SIZE {
        output.push_str(&" ".repeat(SIZE - i - 1));
        output.push('*');
        output.push_str(&"*".repeat(2 * i));
        output.push('\n');
    }

    for i in (0..SIZE - 1).rev() {
        output.push_str(&" ".repeat(SIZE - i - 1));
        output.push('*');
        output.push_str(&"*".repeat(2 * i));
        output.push('\n');
    }

    println!("{}", output);
}
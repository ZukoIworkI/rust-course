use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    println!("Введіть число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");

    let number: u32 = input.trim().parse().expect("Введіть дійсне число");

    if is_prime(number) {
        println!("{} є простим числом.", number);
    } else {
        println!("{} не є простим числом.", number);
    }
}
fn is_palindrome(num: i32) -> bool {
    let s = num.to_string();
    let rev_s: String = s.chars().rev().collect();
    s == rev_s
}

fn main() {
    let number = 12321;
    if is_palindrome(number) {
        println!("{} є паліндромом.", number);
    } else {
        println!("{} не є паліндромом.", number);
    }
}
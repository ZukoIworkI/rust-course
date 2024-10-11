#[test]
fn main() {
    let mut sum = 0;
    for i in -3..2 { // Диапазон от -3 до 1 включительно
        sum += i;
    }

    assert!(sum == -5); // Сумма чисел от -3 до 1 = -5

    for c in 'a'..='z' { // Вывод символов от 'a' до 'z'
        println!("{}", c);
    }
}

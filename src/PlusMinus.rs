use std::io::{self, BufRead};

fn plusMinus(arr: &[i32]) {
    let total = arr.len() as f64;
    let positives = arr.iter().filter(|&&x| x > 0).count() as f64;
    let negatives = arr.iter().filter(|&&x| x < 0).count() as f64;
    let zeros = arr.iter().filter(|&&x| x == 0).count() as f64;

    println!("{:.6}", positives / total);
    println!("{:.6}", negatives / total);
    println!("{:.6}", zeros / total);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}

use std::collections::HashMap;

fn sockMerchant(_n: i32, ar: &[i32]) -> i32 {
    let mut color_count = HashMap::new();

    for &color in ar {
        *color_count.entry(color).or_insert(0) += 1;
    }

    color_count.values().map(|&count| count / 2).sum()
}

fn main() {
    use std::io::{self, BufRead, Write};
    use std::env;
    use std::fs::File;

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    writeln!(&mut fptr, "{}", result).ok();
}

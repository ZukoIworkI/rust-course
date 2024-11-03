fn pageCount(n: i32, p: i32) -> i32 {
    let turns_from_front = p / 2;
    let turns_from_back = (n / 2) - (p / 2);
    turns_from_front.min(turns_from_back)
}

fn main() {
    use std::io::{self, BufRead, Write};
    use std::env;
    use std::fs::File;

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    writeln!(&mut fptr, "{}", result).ok();
}

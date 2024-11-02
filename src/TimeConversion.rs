fn timeConversion(s: &str) -> String {
    let period = &s[8..];
    let mut hour = s[0..2].parse::<u32>().unwrap();

    if period == "AM" {
        if hour == 12 {
            hour = 0;
        }
    } else {
        if hour != 12 {
            hour += 12;
        }
    }

    format!("{:02}:{:02}:{:02}", hour, &s[3..5], &s[6..8])
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

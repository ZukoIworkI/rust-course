fn countingValleys(_steps: i32, path: &str) -> i32 {
    let mut altitude = 0;
    let mut valleys = 0;

    for step in path.chars() {
        if step == 'U' {
            altitude += 1;
            if altitude == 0 {
                valleys += 1;
            }
        } else if step == 'D' {
            altitude -= 1;
        }
    }

    valleys
}

fn main() {
    use std::io::{self, BufRead, Write};
    use std::env;
    use std::fs::File;

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _steps = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(_steps, &path);

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    writeln!(&mut fptr, "{}", result).ok();
}

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut count = [0; 5];
    for &bird_type in arr {
        count[(bird_type - 1) as usize] += 1;
    }
    let mut max_count = 0;
    let mut bird_id = 1;
    for (i, &c) in count.iter().enumerate() {
        if c > max_count {
            max_count = c;
            bird_id = (i + 1) as i32;
        }
    }
    bird_id
}

fn main() {
    use std::io::{self, BufRead, Write};
    use std::env;
    use std::fs::File;

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    writeln!(&mut fptr, "{}", result).ok();
}

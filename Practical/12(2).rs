fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_index = 0;

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    Some((min_index, min_sum))
}

fn main() {
    let vec = gen_random_vector(20);
    if let Some((index, sum)) = min_adjacent_sum(&vec) {
        println!("Minimum adjacent sum is {} at indexes {} and {}", sum, index, index + 1);
    } else {
        println!("Not enough elements to find an adjacent sum.");
    }
}
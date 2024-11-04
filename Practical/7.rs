fn main() {
    let num_trees = 5;
    draw_tree(num_trees);
}

fn draw_tree(num_trees: usize) {
    for level in 0..num_trees {
        draw_triangle(level, num_trees);
    }
}

fn draw_triangle(level: usize, num_trees: usize) {
    let width = 2 * level + 1;
    let padding = num_trees - level - 1;

    for i in 0..(level + 1) {
        let stars = (0..(2 * i + 1)).map(|_| '*').collect::<String>();
        let line = format!("{:^width$}", stars, width = width + padding * 2);
        println!("{}", line);
    }
}
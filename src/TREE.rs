#[test]
fn main() {
    let branch_height = 3; // Number of branches
    let tree_height = 3;   // Height of each tree branch

    for _ in 0..branch_height {
        // Draw the branch top
        for i in 0..tree_height {
            let spaces = tree_height - i - 1;
            let stars = 2 * i + 1;

            // Print spaces before stars
            for _ in 0..spaces {
                print!(" ");
            }

            // Print stars
            for _ in 0..stars {
                print!("*");
            }

            // Move to the next line
            println!();
        }


    }
}

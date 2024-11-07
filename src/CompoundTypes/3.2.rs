fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    of the two chars '中' and '国' occupies 4 bytes, but the slice is a reference
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}
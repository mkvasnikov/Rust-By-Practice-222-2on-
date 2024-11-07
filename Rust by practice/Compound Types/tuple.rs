🌟1
fn main() {
    let _t0: (u8, i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));

    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

🌟2
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}

🌟3
fn main() {
    let too_long_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    println!("too long array: {:?}", too_long_array);
}

🌟4
fn main() {
    let tup = (1, 6.4, "hello");

    
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

🌟5
fn main() {
    let (x, y, z);

    
    (z, x, y) = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

🌟6
fn main() {
    
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

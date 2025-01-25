pub fn scalar_type() {
    println!("----------------------------------> Scalar");
    let x: i32= 5;
    let mut y: u32 = 10;
    let is_rust: bool = true;
    y = y + 5;
    let letter: char = 'R';
    println! ("x: {}", x);
    println! ("y: {}", y);
    println!("Flag: {}", is_rust);
    println!("letter: {}", letter);
}

pub fn compound_type() {
    println!("----------------------------------> Compound");
    let tup: (i32, f64, i32) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Value from tuple: {}, {}, {}", x, y, z);
    println!("five_hun: {}", tup.0);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Value from array: {}", arr[4]);
}

pub fn string_type() {
    println!("----------------------------------> String");
    let mut s: String = String::from("hello");
    s.push_str(", EIEI!!!");
    println!("Sring: {}", s);

    let greeting: &str= "Hello, Rust!";
    println!("Immu string: {}", greeting);
}

pub fn slice_type() {
    // Array ที่แยกส่วนได้
    println!("----------------------------------> Slice-String");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[0..3];
    println!("Slice: {:?}", slice);
}

pub fn vector_type() {
    println!("----------------------------------> Vector");
    let mut v = Vec::new();
    v.push(11);
    v.push(22);

    for i in &v {
        println!("Value in vector: {}", i);
    }
}

pub fn sqare(x: i32, y: i32) -> i32 {
    return x * y;
}
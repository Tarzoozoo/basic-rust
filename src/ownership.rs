pub fn owner_fn() {
    
    println!("----------------------------------> Ownership");
    let s1 = String::from("Hello");
    let s2 = s1;  // s1 ถูกย้ายไปยัง s2

    // println!("{}", s1);  // Error! s1 ไม่สามารถใช้งานได้อีก
    println!("s2: {}", s2);

    // ------------------- Another example -----------------------
    print_string(s2);
    // println!("{}", s2); // Error เพราะ Owner โดนเปลี่ยนเป็น input ใน print_string()
}

fn print_string(input: String) {
    println!("Recieved string: {}", input);
}

pub fn borrow_fn() {

    println!("----------------------------------> Borrowing");
    // ------------------- ยืมแบบอ่าน -----------------------
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);  // ยืม s1 แต่ไม่ได้ย้าย
    println!("The length of '{}' is {}.", s1, len);

    // ------------------- ยืมแบบแก้ไขได้ -----------------------
    let mut m_s1 = String::from("Hello");
    let new_len = calculate_length_mut(&mut m_s1);
    println!("The length of '{}' is {}.", m_s1, new_len);
}

fn calculate_length(s: &String) -> usize {
    s.len()  // เราสามารถใช้ s ได้โดยไม่ต้องย้ายความเป็นเจ้าของ
}

fn calculate_length_mut(s: &mut String) -> usize {
    s.push_str("EIEI");
    s.len()  // เราสามารถใช้ s ได้โดยไม่ต้องย้ายความเป็นเจ้าของ
}

pub fn reference_fn() {

    println!("----------------------------------> Refferences");
    let s = String::from("Hello");
    let r1 = &s;  // immutable reference
    let r2 = &s;  // immutable reference
    // let r3 = &mut s;  // Error! ไม่สามารถมี mutable reference ร่วมกับ immutable references

    println!("{} and {}", r1, r2);
}

// Vector = Dynamic Array
pub fn vector_fn() {
    
    println!("----------------------------------> Vector");
    // 1. Create the Vector
    // สร้าง Vector เปล่า
    let mut number: Vec<i32> = Vec::new();
    number.push(10);
    number.push(20);
    number.push(30);
    println!("Value: {:?}", number);

    // ใช้ vec![] สร้าง Vector
    // ! คือ Macro เพื่อสั่งให้ดำเนินการในจังหวะ Complie time
    let fruits: Vec<&str> = vec!["Apple", "Banana", "Cherry"];
    println!("Fruits: {:?}", fruits);

    // 2. การเข้าถึงข้อมูล
    let first = number[0]; // เข้าถึงข้อมูลที่ตำแหน่งแรก
    println!("Value at first index: {}", first);

    match number.get(5) {
        Some(value) => println!("Value at 5th index: {}", value),
        None => println!("ไม่มีข้อมูลในตำแหน่งที่ 5") // ป้องกันข้อผิดพลาด index เกินขอบเขต

    }

    // 3. For loop
    println!("Foor loop in Vector:");
    for num in &number {
        println!("{}", num); // ใช้ reference เพื่อไม่ให้ ownership เปลี่ยนแปลง
    }

    // 4. การปรับเปลี่ยนข้อมูล
    if let Some(last) = number.last_mut() {
        *last += 5; // เพิ่มค่า 5 ให้ข้อมูลตำแหน่งสุดท้าย
    }
    println!("After modified value at last index: {:?}", number); // [10, 20, 35]

    // 5. การลบข้อมูล
    number.pop(); // ลบข้อมูลตำแหน่งสุดท้าย
    println!("After delete value at last index: {:?}", number);

    number.remove(0); // ลบข้อมูลตำแหน่งแรก
    println!("After delete first index: {:?}", number);

}


// String = ข้อความที่แก้ได้
pub fn string_fn() {

    println!("----------------------------------> String");
    // Create the String
    let mut greeting = String::new();
    greeting.push_str("Hello");
    println!("{}", greeting);
    
    let name = String::from("Rust");
    println!("{}", name);

    // 1. การสร้างและเพิ่มข้อความใน String
    let mut text = String::from("Hello, Rustaceans!");

    // 2. การเข้าถึงข้อมูล
    println!("Text: {}", text);
    println!("First char: {}", text.chars().next().unwrap()); // H
    println!("Part of text: {}", &text[7..17]);

    // 3. การเชื่อมข้อความ
    let add_text = String::from(" Welcome to Rust programing.");
    text.push_str(&add_text); // เชื่อมข้อความ
    println!("After concat text: {}", text);

    // 4. การลบข้อความ
    text.replace_range(7..17, "EIEI"); // ลบและแทนที่ "Rustaceans" ด้วย "Rust"
    println!("After delete text: {}", text);

    text.pop();
    println!("After delete last char: {}", text);

    // 5. การแปลงชนิดระหว่าง String และ &str
    // &str ใช้สำหรับ Read only
    // String สามารถเพิ่มลด แก้ไขได้ คล้ายๆ Vector

    let literal_str = "This is &str"; // String slice
    let mut string_obj = literal_str.to_string(); // แปลงเป็น String
    string_obj.push_str(" EIEI!!!");
    println!("{}", string_obj);

    let string_back_to_str = &string_obj; // แปลงกลับเป็น &str
    println!("{}", string_back_to_str);
}


// รูปแบบของ Dict ที่เก็บเป็น Key-Value
use std::collections::HashMap;
pub fn hash_map_fn() {

    println!("----------------------------------> Hashmap");
    // สร้าง HashMap และเพิ่มข้อมูล
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 80);
    scores.insert("Charlie", 70);

    // 1. การเข้าถึงข้อมูล
    if let Some(score) = scores.get("Alice") { // Pattern ในการ handle error
        println!("Alice's score: {}", score);
    } else {
        println!("No data in Alice");
    }
    // กรณีคีย์ไม่อยู่ใน HashMap
    if let Some(score) = scores.get("David") { // Return เป็น option enum(Some/Nome) สำหรับใช้ handle error
        println!("David's score: {}", score);
    } else {
        println!("No data in David"); // ไม่มีข้อมูลของ David
    }

    // 2. การวนลูป
    println!("All Score:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // 3. การอัปเดตข้อมูล
    scores.insert("Alice", 100); // แทนที่ค่าที่มีอยู่
    println!("Alice's score after update: {}", scores.get("Alice").unwrap()); // unwrap return value ออกมา

    // ใช้ entry API เพื่ออัปเดตแบบมีเงื่อนไข
    scores.entry("David").or_insert(55); // เพิ่มเฉพาะเมื่อไม่มีคีย์นี้
    println!("David's socre: {}", scores.get("David").unwrap());

    // 4. การลบข้อมูล
    scores.remove("Bob");
    println!("After delete Bob:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
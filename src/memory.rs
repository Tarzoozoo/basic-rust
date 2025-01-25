
pub fn stack_mem_fn() {

    println!("----------------------------------> Stack Memmory");
    // จัดการแบบ LIFO (Last In, First Out)

    let x = 5; // ตัวแปรชนิด integer ถูกเก็บใน Stack
    let y = "Hello, Rust!"; // ตัวแปรชนิด &str (String slice) ถูกเก็บใน Stack
    println!("x = {}, y = {}", x, y);
} // เมื่อออกจาก scope = x, y ก็จะโดนนำออกจาก stack ไป y -> x ตามลำดับ

pub fn heap_mem_fn() {

    println!("----------------------------------> Heap Memmory");
    let heap_value = Box::new(42); // ตัวเลข 42 ถูกจัดเก็บใน Heap
    println!("Heap value = {}", heap_value);

    let vec = vec![1, 2, 3, 4, 5]; // ข้อมูลใน Vec ถูกเก็บใน Heap
    println!("Vector values = {:?}", vec);
}
// Arc = Smart pointer ที่เหมือน RC(Read-only) แต่ทำงานใน multi-thread (thread-safe)
// Mutex = Smart pointer ที่เหมือน RefCell ที่อนุญาติให้แค่ Thread เดียวในการแก้ไขข้อมูล
// เพื่อป้องกัน data race

// RwLock = Smart pointer ที่อนุญาติให้ read access พร้อมกันได้หลาย Thread
// อนุญาติให้แค่ Thread เดียวในการแก้ไขข้อมูล

// ปัญหาการแชร์ระหว่าง Thread จะมี 2 ปัญหาใหญ่ๆ
// 1. Data Race: คนสองคนกำลังเข้าถึงข้อมูล และพยายามจัดการอะไรสักอย่างกับข้อมูล
// ในช่วงเวลาเดียวกัน เช่นข้อมูลถูกอ่าน ตอนที่อีกคนนึงกำลังเขียน
// 2. Deadlock: lock แล้วไม่ยอมปล่อย ทำให้คนอื่นแก้ข้อมูลไม่ได้ -> โปรแกรมรันต่อไม่ได้


// Arc = Atomic Reference Counting, ใช้คุณสมบัติ Atomic operation
// รับประกันว่าจะทำงานเสร็จสมบูรณ์ใน “ขั้นตอนเดียว” โดยไม่มีการถูกขัดจังหวะโดยการดำเนินการอื่นใด 
// ไม่ว่าจะเป็นจาก thread อื่น
 
// Mutex = ใช้หลักการของ “lock” คือ ก่อนที่ Thread ใด Thread หนึ่งจะสามารถเข้าถึงข้อมูลภายใน Mutex<T> ได้ 
// Thread นั้นจะต้องทำการ “lock” mutex ก่อน ถ้ามี Thread อื่นถือ lock อยู่ Thread ที่พยายาม lock จะต้อง
// รอจนกว่า lock จะถูกปล่อย (unlock) หลังจากใช้งานข้อมูลเสร็จ Thread จะต้องทำการ “unlock” mutex เพื่อให้ 
// Thread อื่นสามารถเข้าถึงข้อมูลได้

use std::sync::{Arc, Mutex};
use std::thread;

pub fn arc_fn() {

    println!("----------------------------------> Arc");
    // Create an Arc to share data across threads (This allows safe sharing of the vector across threads.)
    let numbers = Arc::new(vec![1, 2, 3, 4, 5]);

    // Create a vector to hold thread handles
    let mut handles = vec![];

    // For loop to print values in another thread
    for i in 0..5 {

        // Clone the Arc to share ownership with each thread
        let numbers_clone = Arc::clone(&numbers);

        // Spawn a thread
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, numbers_clone);
        });

        // Save the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    // handle.join() ensures that the main thread 
    // waits for all spawned threads to complete execution.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads are done.");
}

pub fn mutex_fn() {

    println!("----------------------------------> Mutex");
    let counter = Arc::new(Mutex::new(0)); // ใช้ Arc เพื่อแชร์ Mutex ระหว่าง Thread

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // clone Arc เพิ่มตัวนับอ้างอิง

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // ล็อค Mutex
            *num += 1; // เพิ่มค่า
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // รอให้ Thread เสร็จสิ้น
    }

    println!("Result: {}", *counter.lock().unwrap()); // แสดงผลลัพธ์
}
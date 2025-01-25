use std::thread;

pub fn thread_fn() {

    println!("----------------------------------> Threads");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
        }
    });

    for i in 1..5 {
        println!("Main: {}", i);
    }

    handle.join().unwrap(); // รอให้ thread ทำงานเสร็จก่อนโปรแกรมจบ
}


pub fn handle_thread_fn() {

    // move closure = การ ย้าย ownership ไปยัง Thread อื่น
    // เพื่อป้องกันปัญหา ตัวแปรที่ถูกยืมมาหมดอายุ ใน Thread เดิม

    // การย้าย ownership จึงเป็นการรับประกันว่าข้อมูลที่ closure 
    // ต้องการใช้งานนั้นยังคงมีอยู่และ valid ใน thread ใหม่

    println!("----------------------------------> Handle Thread");
    let name = String::from("Rust");

    thread::spawn(move || {
        println!("Hello, {} from a thread!", name);
    }).join().unwrap();

    // println!("Name: {}", name); // error: value borrowed here after move
}


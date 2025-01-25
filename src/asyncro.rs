// โดยหลักการทำงานของ Asynchronous ใน Rust คือ Rust จะใช้แนวคิดของ Futures 
// และ Async/Await ในการจัดการ Asynchronous Programming

// Future คือ Object ที่แทนผลลัพธ์ของการคำนวณแบบ Asynchronous ซึ่งอาจจะยังไม่เสร็จสิ้นในทันที \
// เมื่อการคำนวณเสร็จสิ้น Future จะให้ผลลัพธ์ออกมา

// Async/Await เป็น Syntax ที่ช่วยให้เขียน code Asynchronous ได้ง่ายขึ้น
// Async: ใช้กับ Function หรือ Block เพื่อระบุว่า code ภายในเป็นแบบ Asynchronous
// Await: ใช้ภายใน Async Function เพื่อรอให้ Future เสร็จสิ้น

use tokio::time::{sleep, Duration};

#[tokio::main]
pub async fn async_fn() {

    println!("----------------------------------> Asynchronous");
    println!("Start");

    // เรียกใช้ function  asynchronous พร้อมกัน
    let task1 = async {
        println!("Task 1 start");
        sleep(Duration::from_secs(2)).await; // รอ 2 วินาที
        println!("Task 1 end");
        "Result from Task 1" // คืนค่าจาก Task
    };

    let task2 = async {
        println!("Task 2 start");
        sleep(Duration::from_secs(1)).await; // รอ 1 วินาที
        println!("Task 2 end");
        "Result from Task 2"
    };

    // รอให้ Task ทั้งสองเสร็จสิ้นพร้อมกัน
    let (result1, result2) = tokio::join!(task1, task2);

    println!("{}", result1);
    println!("{}", result2);

    println!("End");
}
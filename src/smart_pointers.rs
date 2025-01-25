pub fn box_fn() {

    println!("----------------------------------> Box");
    let x = Box::new(10); // จัดเก็บค่า 10 บน heap
    println!("ค่าใน Box คือ: {}", x);
}


use std::rc::Rc;
pub fn rc_fn() {

    println!("----------------------------------> Rc");
    // สามารถแชร์ ownership ของค่าบน heap ระหว่างกันได้ 
    // โดยจะนับจำนวน reference ที่ชี้ไปยังค่าดังกล่าว 
    // และเมื่อไม่มี reference (reference count = 0)ใดเหลืออยู่ 
    // ระบบจะปลดปล่อยหน่วยความจำบน heap โดยอัตโนมัติ

    let value = Rc::new(10); // สร้าง Rc ที่เก็บค่าบน heap
    let a = Rc::clone(&value); // เพิ่ม reference ไปยังค่าใน Rc
    let b = Rc::clone(&value); // เพิ่ม reference อีกตัว

    println!("ค่าใน a: {}", a);
    println!("ค่าใน b: {}", b);
    println!("Reference count: {}", Rc::strong_count(&value)); // แสดงจำนวน reference

}

use std::cell::RefCell;
pub fn refcell_fn() {

    println!("----------------------------------> RefCell");

    let shared_vec = RefCell::new(vec![1, 2, 3]);

    // Borrow the value immutably
    {
        let borrowed_vec = shared_vec.borrow();
        println!("Current values: {:?}", *borrowed_vec);
    } // `borrowed_vec` goes out of scope here, so the borrow ends.


    // Borrow the value mutably
    {
        let mut borrowed_vec_mut = shared_vec.borrow_mut();
        borrowed_vec_mut.push(4);
        borrowed_vec_mut.push(5);
    } // `borrowed_vec_mut` goes out of scope here, so the mutable borrow ends.

    // Check the updated values
    println!("Updated values: {:?}", *shared_vec.borrow());
}
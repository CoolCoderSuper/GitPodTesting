use std::{thread, time::Duration};

pub fn simple_threading() {
    let t = thread::spawn(|| {
        for i in 1..7{
            println!("Thread: {i}");
            thread::sleep(Duration::from_secs(1))
        }
    });
    for i in 1..5{
        println!("Main Thread: {i}");
        thread::sleep(Duration::from_millis(500))
    }
    t.join().unwrap();
    let t_val = thread::spawn(|| -> i32 {
        thread::sleep(Duration::from_secs(2));
        1
    });
    let res = t_val.join().expect("");
    println!("{res}")
}
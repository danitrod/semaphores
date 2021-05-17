use semaphores::{check_occurrence_order, Semaphore};
use std::sync::{Arc, Mutex};
use std::thread;

// Thread A
// a1, a2

// Thread B
// b1, b2

// Problem: guarantee the order a1 < b2 and b1 < a2

fn main() {
    let mut t1_a1done = Semaphore::new(&mut 0);
    let mut t2_a1done = t1_a1done.clone();

    let mut t1_b1done = Semaphore::new(&mut 0);
    let mut t2_b1done = t1_b1done.clone();

    let t1_order = Arc::new(Mutex::new(Vec::new()));
    let t2_order = t1_order.clone();

    thread::spawn(move || {
        // a1
        t1_order.lock().unwrap().push("a1");
        t1_a1done.signal();

        t1_b1done.wait();

        // a2
        t1_order.lock().unwrap().push("a2");
    });

    // b1
    t2_order.lock().unwrap().push("b1");
    t2_b1done.signal();

    t2_a1done.wait();

    // b2
    t2_order.lock().unwrap().push("b2");

    assert!(check_occurrence_order(
        &t2_order.lock().unwrap(),
        "a1",
        "b2"
    ));

    assert!(check_occurrence_order(
        &t2_order.lock().unwrap(),
        "b1",
        "a2"
    ));
}

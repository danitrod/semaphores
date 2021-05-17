use semaphores::Semaphore;
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let mut sem = Semaphore::new(&mut 0);
    let mut sem2 = sem.clone();

    let mut handles = Vec::new();

    let order_counter = Arc::new(Mutex::new(Vec::<usize>::new()));
    let order_counter_1 = order_counter.clone();
    let order_counter_2 = order_counter.clone();

    // Thread #1, should end first
    handles.push(thread::spawn(move || {
        sem.wait();
        order_counter_1.lock().unwrap().push(1);
    }));

    thread::sleep(time::Duration::from_millis(1));

    // Thread #2, should end second
    handles.push(thread::spawn(move || {
        order_counter_2.lock().unwrap().push(2);
        sem2.signal();
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(*order_counter.lock().unwrap(), vec![2, 1]);
}

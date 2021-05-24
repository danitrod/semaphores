use semaphores::Semaphore;
use std::thread;

// Multiplex
// Problem: same as mutex, but generalizing for N threads to
// be able to access the critical section at the same time.

#[derive(Clone)]
struct RawPtrSend(*mut i32);
unsafe impl Send for RawPtrSend {}

fn main() {
    let mut n = 5;
    let mut mutex = Semaphore::new(&mut n);

    let count = RawPtrSend(&mut 0 as *mut i32);

    let mut handles = Vec::new();

    for _ in 0..n {
        let mut mutex = mutex.clone();
        let count = count.clone();
        handles.push(thread::spawn(move || {
            // Wait on the critical section
            mutex.wait();
            unsafe {
                *count.0 += 1;
            }
            mutex.signal();
        }));
    }

    // Wait on the critical section
    mutex.wait();
    unsafe {
        *count.0 += 1;
    }
    mutex.signal();

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        assert_eq!(*count.0, n + 1);
    }
}

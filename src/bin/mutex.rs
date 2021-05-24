use semaphores::Semaphore;
use std::thread;

// Thread A
// count = count + 1

// Thread B
// count = count + 1

// Mutex
// Problem: guarantee that both threads add to the count variable as above

#[derive(Clone)]
struct RawPtrSend(*mut i32);
unsafe impl Send for RawPtrSend {}

fn main() {
    let mut mutex = Semaphore::new(&mut 1);
    let mut mutex2 = mutex.clone();

    let count = RawPtrSend(&mut 0 as *mut i32);
    let count2 = count.clone();

    thread::spawn(move || {
        // Wait on the critical section
        mutex2.wait();
        unsafe {
            *count2.0 += 1;
        }
        mutex2.signal();
    })
    .join()
    .unwrap();

    // Wait on the critical section
    mutex.wait();
    unsafe {
        *count.0 += 1;
    }
    mutex.signal();

    unsafe {
        assert_eq!(*count.0, 2);
    }
}

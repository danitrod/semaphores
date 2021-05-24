use semaphores::Semaphore;
use std::thread;

// Problem: use semaphores to enforce the following constraints:

// 1. Any number of readers can be in the critical section simultaneously.
// 2. Writers must have exclusive access to the critical section.

#[derive(Clone)]
struct Lightswitch {
    counter: RawPtrSend,
    mutex: Semaphore,
}

unsafe impl Send for Lightswitch {}

impl Lightswitch {
    pub fn new() -> Self {
        Self {
            counter: RawPtrSend(&mut 0 as *mut i32),
            mutex: Semaphore::new(&mut 1),
        }
    }

    pub fn lock(&mut self, semaphore: &mut Semaphore) {
        self.mutex.wait();
        unsafe {
            *self.counter.0 += 1;
            if *self.counter.0 == 1 {
                semaphore.wait();
            }
        }
        self.mutex.signal();
    }

    pub fn unlock(&mut self, semaphore: &mut Semaphore) {
        self.mutex.wait();
        unsafe {
            *self.counter.0 -= 1;
            if *self.counter.0 == 0 {
                semaphore.signal();
            }
        }
        self.mutex.signal();
    }
}

#[derive(Clone)]
struct RawPtrSend(*mut i32);
unsafe impl Send for RawPtrSend {}

fn main() {
    let lightswitch = Lightswitch::new();
    let room_empty = Semaphore::new(&mut 1);
    let mut handles = Vec::new();

    let n_threads = 5;
    let shared_var = RawPtrSend(&mut 5 as *mut i32);

    for i in 0..n_threads {
        let shared_var = shared_var.clone();
        let mut room_empty = room_empty.clone();
        let mut lightswitch = lightswitch.clone();
        handles.push(thread::spawn(move || {
            if i % 2 == 0 {
                // writers
                room_empty.wait();
                unsafe {
                    *shared_var.0 += 5;
                }
                room_empty.signal();
            } else {
                // readers
                lightswitch.lock(&mut room_empty);
                unsafe {
                    println!("Val is {}", *shared_var.0);
                }
                lightswitch.unlock(&mut room_empty);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        println!("Final val: {}", *shared_var.0);
        assert_eq!(*shared_var.0, 5 * (n_threads - 1));
    }
}

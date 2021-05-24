pub trait OccurrenceOrder<T> {
    /// Checks if given first argument occurs earlier than the second.
    ///
    /// Returns `false` if the first argument is not found.
    fn check_occurrence_order(&self, first: T, second: T) -> bool;
}

impl<T: PartialEq> OccurrenceOrder<T> for Vec<T> {
    fn check_occurrence_order(&self, first: T, second: T) -> bool {
        let mut found_first = false;
        let mut found_second = false;

        for val in self.iter() {
            if *val == first {
                found_first = true;
            }
            if *val == second {
                if found_first {
                    found_second = true;
                    break;
                }
            }
        }

        return found_second;
    }
}

#[derive(Clone)]
struct RawPtrSend(*mut i32);
unsafe impl Send for RawPtrSend {}

/// **Warning!**
/// This Semaphore implementation is not ready for production use.
///
/// It contains unsafe code that may cause unexpected behavior.
#[derive(Clone)]
pub struct Semaphore {
    value: RawPtrSend,
}

impl Semaphore {
    pub fn new(value: &mut i32) -> Self {
        Self {
            value: RawPtrSend(value as *mut i32),
        }
    }

    pub fn signal(&mut self) {
        unsafe {
            *self.value.0 += 1;
        }
    }

    pub fn wait(&mut self) {
        unsafe {
            *self.value.0 -= 1;

            while *self.value.0 < 0 {}
        }
    }

    pub fn show(&self) -> i32 {
        unsafe { *self.value.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::{thread, time};

    #[test]
    fn semaphores() {
        let mut sem = Semaphore::new(&mut 0);
        let mut sem2 = sem.clone();

        let mut handles = Vec::new();

        let order_counter = Arc::new(Mutex::new(Vec::<usize>::new()));
        let order_counter_1 = order_counter.clone();
        let order_counter_2 = order_counter.clone();

        // Thread #1, should be blocked
        handles.push(thread::spawn(move || {
            sem.wait();
            order_counter_1.lock().unwrap().push(1);
        }));

        thread::sleep(time::Duration::from_millis(1));

        // Thread #2, will unblock #1
        handles.push(thread::spawn(move || {
            order_counter_2.lock().unwrap().push(2);
            sem2.signal();
        }));

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*order_counter.lock().unwrap(), vec![2, 1]);
    }

    #[test]
    fn vec_ordering() {
        // Correct order with number
        assert!(vec![5, 4, 3].check_occurrence_order(5, 3));

        // Wrong order
        assert!(!vec!["c1", "b1", "a1"].check_occurrence_order("a1", "c1"));

        // Correct order with string
        assert!(vec!["c1", "b1", "a1"].check_occurrence_order("c1", "a1"));

        // Not found
        assert!(!vec!["c1", "b1", "a1"].check_occurrence_order("z", "p"));
    }
}

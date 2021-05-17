/*
! Warning!
This Semaphore implementation is not ready for production use.

Cloning a raw pointer may not work when compiling with optimizations.
*/

#[derive(PartialEq)]
enum OrderStatus<T: PartialEq> {
    Waiting(Vec<T>),
    Failed,
    Ok,
}

impl<T: PartialEq> OrderStatus<T> {
    pub fn is_ok(self) -> bool {
        self == OrderStatus::Ok
    }
}

pub fn check_occurrence_order<T: PartialEq>(order: &Vec<T>, first: T, second: T) -> bool {
    order
        .iter()
        .fold(OrderStatus::Waiting(Vec::new()), |acc, v| match acc {
            OrderStatus::Failed | OrderStatus::Ok => acc,
            OrderStatus::Waiting(values) => {
                if *v == first {
                    OrderStatus::Ok
                } else if *v == second {
                    OrderStatus::Failed
                } else {
                    let mut new_values = values;
                    new_values.push(v);
                    OrderStatus::Waiting(new_values)
                }
            }
        })
        .is_ok()
}

#[derive(Clone)]
struct RawPtrSend(*mut i32);
unsafe impl Send for RawPtrSend {}

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
        assert!(check_occurrence_order(&vec![5, 4, 3], 5, 3));
        assert!(check_occurrence_order(&vec!["c1", "b1", "a1"], "c1", "a1"));
    }
}

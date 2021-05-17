use semaphores::Semaphore;

fn main() {
    println!("Starting");
    let mut val = 0;
    let mut sem = Semaphore::new(&mut val);

    println!("Semaphore: {}", sem.show());
    sem.signal();
    sem.signal();
    sem.signal();
    println!("Semaphore: {}", sem.show());
    sem.wait();
    println!("Semaphore: {}", sem.show());
    sem.wait();
    sem.wait();
    println!("Semaphore: {}", sem.show());
}

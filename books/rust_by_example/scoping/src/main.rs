#![allow(unused_variables)]

fn main() {
    create_box();
    for _ in 0..1_000 {
        create_box();
    }

    let x = ToDrop;
    println!("Made a ToDrop!");
}


/// Create a couple of integers on the heap
fn create_box() {
    let box1 = Box::new(3_u64);
    let box2 = Box::new(5_u64);
}


/// ToDrop
struct ToDrop;

/// Destructors not needed, but we can implement `drop()` if we have extra
/// clean-up to do.
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

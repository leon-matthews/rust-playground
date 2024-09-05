/*!
Smart pointers implement the `Deref` and `Drop` traits.

Deref to get access to contents and Drop to remove owned memory.

* `Box<T>` allocates values on the heap
* `Rc<T>` & `Arc<T>` implements reference counting for mulitple ownership
* `Ref<T>` & `RefMut<T>` (accessed via `RefCell<T>`) enforces borrowing rules
  at runtime instead of compile time.
*/

fn main() {
    smart_box();
}


/**
`Box<T>` force data onto the heap with a fixed-size pointer.

Use when you need an exact size on the stack, or want to transfer ownership
to a large amount of data, or just need a trait object.

When the box goes out of scope both the box itself (on the stack) and the data
that it points to (on the heap) are deallocated.
*/
fn smart_box() {
    let mut b = Box::new(5);
    *b += 1;                            // Manual deferencing required
    println!("b = {b}");                // Automatic dereferencing
}

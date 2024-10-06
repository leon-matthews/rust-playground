
fn main() {
    for i in Fibonacci::new().take(40) {
        println!("> {}", i);
    }
}


// Iterator's state
struct Fibonacci {
    curr: u64,
    next: u64,
}


impl Fibonacci {
    /// Returns (an almost infinite*) Fibonacci sequence generator
    /// * It won't finish, just crash when it exceeds its `Item` type capacity.
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}


impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(self.curr)
    }
}

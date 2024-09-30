#![allow(dead_code)]
#![allow(unused_variables)]


fn main() {
    for i in fibonacci().take(40) {
        println!("> {}", i);
    }
}


// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}


// Hold state
struct Fibonacci {
    curr: u64,
    next: u64,
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


fn main() {
    // You can optionally experiment here.
}


#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();

        // Mutable borrow to 'y' for two lines
        let y = &mut x;
        y.push(42);

        // Mutable borrow to 'z' for two lines
        let z = &mut x;
        z.push(13);

        // Lifetimes of both borrows finish, x is accessible again.
        assert_eq!(x, [42, 13]);
    }
}

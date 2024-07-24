
fn main() {
    // You can optionally experiment here.
}


#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Use a tuple index to access the second element of `numbers`
        let second = numbers.1;
        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

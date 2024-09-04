/**
Iterators are produced, adapted, then consumed.

They are lazy, in the sense that they perform no action without being consumed.
*/


fn main() {
    basics();
    produce_adapt_consume();
}


fn basics() {
    let v: Vec<u8> = vec![2, 3, 5, 7];

    // iter() is a method that replaces awkward call to `(&v).into_iter()`
    // `mut` needed because iterator's internal state has to change
    let mut i = v.iter();

    // Takes ownership
    //~ v.into_iter()

    // Takes mutable reference
    //~ v.iter_mut()                    // (&mut v).into_iter()

    // Takes immutable refereces
    //~ v.iter()                        // (&v).into_iter()

    // Values returned by next are immutable references (this time!)
    assert_eq!(i.next(), Some(&2));     // Option<&u8>
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), Some(&5));
    assert_eq!(i.next(), Some(&7));

    // Iterator returns `Option::None` when iteration finished
    assert_eq!(i.next(), None);
    assert_eq!(i.next(), None);
}


fn produce_adapt_consume() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let total: u32 = v
        .iter()                         // Produce
        .map(|x| x * 10)                // Adapt
        .sum();                         // Consume
    println!("{total}");
}

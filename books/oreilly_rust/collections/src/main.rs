#![allow(unused_variables)]

use std::collections::*;


/**
There are eight standard collections, all generic types:

1. Vec<T>, a growable array like Python's list or the vector in C++.
2. VecDeque<T>, a double ended queue.
3. LinkedList<T>, a double-linked list. Rarely used.
4. BinaryHeap<T> where T: Ord, a max heap or a priority queue.
5. HashMap<K, V> where K: Eq + Hash, a key-value hash table mapping.
6. BTreeMap<K, V> where K: Ord, like HashMap but keeps elements ordered.
7. HashSet<T> where T: Eq + Hash, an unordered hash-based set.
8. BTreeSet<T> where T: Ord, a sorted set.

The most-used are Vec<T>, HashMap<K, V>, and HashSet<T>.
*/
fn main() {
    vec_macro();
    vec_slice_access();
    vec_grow_and_shrink();
    slice_sort_search();
    vecdeque_example();
    priority_queue();
}


/// Create vectors using `vec!` macro.
fn vec_macro() {
    // Empty vector
    let v: Vec<i32> = vec![];

    // Again, but type inferred by later operations
    let mut v = vec![];
    v.push(42);                                     // Vec<i32>

    // Create from contents
    let v = vec!["The", "quick", "brown", "fox"];   // Vec<&str>

    // Repeat first element, note semi-colon
    let buffer = vec![0_u8; 1024];                  // 1KiB of zeros
    println!("Buffer is {} bytes long", buffer.len());
}


/// Access values inside vertors and slices
fn vec_slice_access() {
    // Index operators can get elements or slices, and references to same.
    // Requires `usize` arguments, and will panic if index out of bounds.
    let words = vec!["The", "quick", "brown", "fox"];
    assert_eq!(words[0], "The");
    assert_eq!(words[1..3], ["quick", "brown"]);

    // Using slice access methods are safer - they also work
    // on arrays and vectors too.

    // first() returns reference to first element, Option<&T>
    let maybe_first = words.first();
    if let Some(first) = maybe_first {
        print!("{first} ");
    }

    // last(), is *almost* the same as first()
    let maybe_last = words.last();
    println!("{maybe_last:?}");

    // get(usize), returns Option<&T> to element, if any.
    assert_eq!(words.get(3), Some("fox").as_ref());
    assert_eq!(words.get(123), None);

    // Also `slice.len()` & `slice.is_empty()`
}


fn vec_grow_and_shrink() {
    fn info<T>(v: &Vec<T>) -> String {
        format!("(length: {}, capacity: {})", v.len(), v.capacity())
    }

    // Vectors usually have plenty of spare space
    let mut v = vec![];
    assert!(v.is_empty());
    println!("New vector {}", info(&v));

    v.push(42);
    println!("Push integer {}", info(&v));

    // We can reserve space for new vector
    let v: Vec<u8> = Vec::with_capacity(1024);
    println!("Vec::with_capacity(1024) {}", info(&v));

    // We can reserve space for an existing vector too
    let mut v: Vec<_> = (1..100).collect();
    println!("(1..100).collect() {}", info(&v));

    // Grow, leaving no extra space
    v.reserve_exact(200);
    println!("v.reserve_exact(200) {}", info(&v));

    // Grow, maybe leaving extra capacity
    v.reserve(300);
    println!("v.reserve(300) {}", info(&v));

    // Shrink down to fit contents
    v.shrink_to_fit();
    println!("v.shrink_to_fit() {}", info(&v));
}


fn slice_sort_search() {
    // slice.sort(), present only when element type implements `Ord`
    let mut v = vec![2, 5, 1, 9, 4, 7, 6, 10, 3, 8];
    v.sort();
    assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // slice.binary_search(), only works on sorted slice
    // Returns Ok(index) of existing element, or Err(index) where value
    // should be inserted to preserve order.
    let maybe_index = v.binary_search(&5);
    println!("5 found at index {}", maybe_index.unwrap());

    // slice.reverse(), reverses in place
    v.reverse();
    assert_eq!(v, [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);

    // slice.contains(), short-circuit linear search, no sort needed.
    assert_eq!(v.contains(&5), true);
}


/**
VecDeque<T> implements circular buffer to efficiently push elements to
either end.
*/
fn vecdeque_example() {
    let mut d = VecDeque::from(vec!['B', 'C', 'D']);
    d.push_front('A');
    d.push_back('E');
    println!("{:?}, length {}, capacity {}", d, d.len(), d.capacity());
    assert_eq!(d, ['A', 'B', 'C', 'D', 'E']);
}


/**
BinaryHeap<T> collection where elements are kept loosly organised so that the
greatest value always bubbles to the front of the queue.
*/
fn priority_queue() {
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4, 7]);
    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(7));
}

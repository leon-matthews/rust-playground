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
    hashmap_and_btreemap();
    hashset_and_btreeset();
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
`VecDeque<T>` implements circular buffer to efficiently push elements to
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
`BinaryHeap<T>` collection where elements are kept loosly organised so that the
greatest value always bubbles to the front of the queue.
*/
fn priority_queue() {
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4, 7]);
    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(7));
}


/**
`HashMap<K, V>` and `BTreeMap<K, V>` are similar with different representations.

The former is a little faster, the latter keeps its elements in (sorted, not
insertion) order.
*/
fn hashmap_and_btreemap() {
    // Top 10 words in Shakespeare's complete works
    let word_counts = vec![
        ("the", 28_944),
        ("and", 27_120),
        ("i", 21_120),
        ("to", 20_136),
        ("a", 14_945),
        ("you", 13_989),
        ("my", 12_949),
        ("in", 11_513),
        ("that", 11_488),
        ("is", 9_545),
    ];

    // HashMap<K, V> from vector of tuples.
    // Note use of entry API that effectively borrows `mut ref` to value.
    let mut hash_map: HashMap<_, _> = word_counts.clone().into_iter().collect();
    hash_map.insert("is", 9_545);
    hash_map.entry("not").or_insert(8_855);
    hash_map.entry("with").or_insert(8_293);
    hash_map.entry("me").or_insert(8_043);
    hash_map.entry("it").or_insert(8_003);
    println!("HashMap =====");
    for (k, v) in &hash_map {
        println!("{k:<4} -> {v:>5}");
    }
    println!();

    // BTreeMap<K, V> from same vector.
    let mut btree_map: BTreeMap<_, _> = word_counts.into_iter().collect();
    btree_map.insert("is", 9_545);
    btree_map.entry("not").or_insert(8_855);
    btree_map.entry("with").or_insert(8_293);
    btree_map.entry("me").or_insert(8_043);
    btree_map.entry("it").or_insert(8_003);
    println!("BTreeMap ====");
    for (k, v) in &btree_map {
        println!("{k:<4} -> {v:>5}");
    }
}


/**
Sets are collections of values arranged for fast membership testing.

A `BTreeSet<T>` keeps value in lexiographic order.
*/
fn hashset_and_btreeset() {
    let mut set = HashSet::<String>::new();
    set.insert(String::from("carrot"));
    set.insert(String::from("banana"));
    set.insert(String::from("apple"));
    for entry in &set {
        print!("{entry} ");
    }
    println!();

    let mut bset = BTreeSet::<String>::new();
    bset.insert(String::from("carrot"));
    bset.insert(String::from("banana"));
    bset.insert(String::from("apple"));
    bset.insert(String::from("durian"));
    for entry in &bset {
        print!("{entry} ");
    }
    println!();
}

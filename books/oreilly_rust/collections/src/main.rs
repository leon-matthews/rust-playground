
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
    println!("Hello, world!");
}

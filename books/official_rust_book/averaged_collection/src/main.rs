
use averaged_collection::AveragedCollection;


fn main() {
    let mut ages = AveragedCollection::new();
    ages.add(12);
    ages.add(16);
    ages.add(47);
    ages.add(48);
    println!("{}", ages.average());

    ages.add(67);
    println!("{}", ages.average());
    assert_eq!(ages.remove(), Some(67));
    println!("{}", ages.average());
}

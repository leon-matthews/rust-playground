
/**
A strings capacity grows like a vector's when increased in size.
*/
fn main() {
    let mut name = String::from("Leon");
    dbg!(name.len(), name.capacity());

    name += " Matthews";
    dbg!(name.len(), name.capacity());

    name.push_str("!");
    dbg!(name.len(), name.capacity());
}

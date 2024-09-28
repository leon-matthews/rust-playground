
use binary_tree::BinaryTree;


fn main() {
    let list = vec![
        "Mercury",
        "Venus",
        "Earth",
        "Mars",
        "Jupiter",
        "Saturn",
        "Uranus",
        "Neptune",
    ];

    let planets = BinaryTree::from_iter(list);
    println!("There are {} planets in our solar system:", planets.len());
    for planet in &planets {
        println!("{planet}");
    }
}

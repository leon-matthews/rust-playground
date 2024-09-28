
use binary_tree::BinaryTree;


fn main() {
    let planets = vec![
        "Mercury",
        "Venus",
        "Earth",
        "Mars",
        "Jupiter",
        "Saturn",
        "Neptune",
    ];
    println!("{planets:?}");

    let mut tree = BinaryTree::Empty;
    for planet in planets {
        tree.add(planet);
    }
    println!("{tree:#?}");
}

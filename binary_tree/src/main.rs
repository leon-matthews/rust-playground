
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
    let mut planets = BinaryTree::Empty;
    for planet in list {
        planets.add(String::from(planet));
    }

    println!("There are {} planets in our solar system:", planets.len());
    for planet in &planets {
        println!("{planet}");
    }
}

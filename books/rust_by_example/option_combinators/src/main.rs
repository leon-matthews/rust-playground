#![allow(dead_code)]
#![allow(unused_variables)]


#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}


#[derive(Debug)]
struct Peeled(Food);


#[derive(Debug)]
struct Chopped(Food);


#[derive(Debug)]
struct Cooked(Food);


/// Peel the food, if any.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}


/// Chop the peeled food, if possible
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    // Use pattern to extract food, then rewrap in Chopped
    peeled.map(|Peeled(f)| Chopped(f))
    //~ match peeled {
        //~ Some(Peeled(food)) => Some(Chopped(food)),
        //~ None => None,
    //~ }
}


/// Cook the chopped food
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(f)| Cooked(f))
}


/// Peel, chop, and cook  - all in one!
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|food| Peeled(food))
        .map(|Peeled(peeled)| Chopped(peeled))
        .map(|Chopped(chopped)| Cooked(chopped))
}


fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato: Option<Food> = None;

    println!("{:?}", cook(chop(peel(apple))));
    println!("{:?}", process(carrot));
    println!("{:?}", process(potato));
}

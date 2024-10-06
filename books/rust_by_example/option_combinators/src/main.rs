#![allow(dead_code)]
#![allow(unused_variables)]

/**
From the Rust glossary:

    Combinators are higher-order functions that apply only functions and
    earlier defined combinators to provide a result from its arguments. They
    can be used to manage control flow in a modular fashion.

*/

fn main() {
    option_map();
    option_and_then();
}


/**
Maps an Option<T> to Option<U> by applying a function to a contained value
(if Some) or returns None (if None).

https://doc.rust-lang.org/std/option/enum.Option.html#method.map
*/
fn option_map() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato: Option<Food> = None;

    println!("{:?}", cook(chop(peel(apple))));
    println!("{:?}", process(carrot));
    println!("{:?}", process(potato));
}


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

    // Same as above, but more verbose
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


/**
Option::and_then()

Returns None if the option is None, otherwise calls f with the wrapped value
and returns the result. Some languages call this operation flatmap.

https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then
*/
fn option_and_then() {
    let (cordon_bleu, steak, sushi) = (
        FancyFood::CordonBleu,
        FancyFood::Steak,
        FancyFood::Sushi
    );

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}


#[derive(Debug)]
enum FancyFood {
    CordonBleu,
    Steak,
    Sushi,
}


#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}


fn eat(food: FancyFood, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}


/// To make a dish, we need both the recipe and the ingredients.
fn cookable_v1(food: FancyFood) -> Option<FancyFood> {
    match have_recipe(food) {
        None       => None,
        Some(food) => have_ingredients(food),
    }
}


/// Using map works *but* will wrap `Option<FancyFood>` to `Option<Option<FancyFood>>`
/// So we need `Option::flatten()` to remove the outer `Option`.
/// Works, but is ugly.
fn cookable_v2(food: FancyFood) -> Option<FancyFood> {
    have_recipe(food).map(have_ingredients).flatten()
}


/// This can conveniently be rewritten more compactly with `and_then()`
/// Note that we are passing a plain function rather than a closure.
fn cookable_v3(food: FancyFood) -> Option<FancyFood> {
    have_ingredients(food).and_then(have_recipe)
}


/// We don't have the ingredients to make Sushi.
fn have_ingredients(food: FancyFood) -> Option<FancyFood> {
    match food {
        FancyFood::Sushi => None,
        _ => Some(food),
    }
}

/// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: FancyFood) -> Option<FancyFood> {
    match food {
        FancyFood::CordonBleu => None,
        _ => Some(food),
    }
}

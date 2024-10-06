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
    option_defaults();
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


/**
Unpack an Option and fall back on a default if it is None.

There are several ways to do this.
*/
fn option_defaults() {
    option_or();
    option_or_else();
    option_get_or_insert();
    option_get_or_insert_with();
}


#[derive(Debug)]
enum Fruit
{
    Apple,
    Orange,
    Banana,
    KiwiFruit,
    Lemon,
}


/**
`Option::or()` is chainable, evaluates eagerly, keeps empty value intact.

https://doc.rust-lang.org/core/option/enum.Option.html#method.or
*/
fn option_or() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);

    // Even though `apple` wasn't used, it was still moved out by `or`
    // error[E0382]: borrow of moved value: `apple`
    //~ println!("Variable apple was moved, so this line won't compile: {:?}", apple);
}

/**
`Option::or_else()` is chainable, evaluates lazily, keeps empty value intact.

https://doc.rust-lang.org/core/option/enum.Option.html#method.or_else
*/
fn option_or_else() {
    let no_fruit: Option<Fruit> = None;

    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::KiwiFruit)
    };

    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    // Note that `get_lemon_as_fallback` is never called
    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);
}


/**
`Option::get_or_insert()` evaluates eagerly, modifies empty value in place.

To make sure that an Option contains a value, we can use get_or_insert to
modify it in place with a fallback value, as is shown in the following example.
Note that get_or_insert eagerly evaluates its parameter, so variable apple is
moved.

As opposed to `Option::insert()` which always replaces contained value.

https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert
*/
fn option_get_or_insert() {
    // Starts off as None
    let mut my_fruit: Option<Fruit> = None;
    println!("my_fruit was: {:?}", my_fruit);

    // About to be (eagerly) moved into function
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);

    // None value changed in place
    println!("my_fruit is: {:?}", my_fruit);
}


/**
`Option::get_or_insert_with()` evaluates lazily, modifies empty value in place.
*/
fn option_get_or_insert_with() {
    let mut my_fruit: Option<Fruit> = None;
    println!("my_fruit was: {:?}", my_fruit);

    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };

    // Closure called (and value modified) ONLY if `my_fruit` is None.
    let first_available_fruit = my_fruit
        .get_or_insert_with(get_lemon_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);

    // `get_lemon_as_fallback()` is NOT called in this case
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);
}

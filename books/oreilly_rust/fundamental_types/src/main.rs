
fn main() {
    closures_are_variables();
    functions_are_variables(hello);
}


fn closures_are_variables() {
    let make_greeting = |who| format!("Hello, {}!", who);
    println!("{}", make_greeting("world"));
}


/// Plain function passed into `functions_are_variables()`
fn hello(who: &str) {
    println!("Hello, {}", who);
}

/// Accepts the full-fledged function above as an argument (see `main()`)`
fn functions_are_variables(greeting: fn(&str)) {
    greeting("world!");
}

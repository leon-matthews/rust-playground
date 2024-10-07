/**
How to deal with iterating over `Result`s.
*/

fn main() {
    iterate_results();
    ignore_failed();
    collect_failed();
    fail_collect();
    partition_failures();
}

fn iterate_results() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();

    // vec![Err, Ok, Ok]
    println!("Results: {:?}", numbers);
}

/**
Ignore the failed items with `filter_map()`

`filter_map()` calls a function and filters out the results that are None.
*/
fn ignore_failed() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}

/**
Collect the failed items with `map_err()` and `filter_map()`

`map_err()` calls a function with the error, so by adding that to the previous
`filter_map()` solution we can save them off to the side while iterating.
*/
fn collect_failed() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

/**
Fail the entire operation with collect()

`Result` implements `FromIterator` so that a vector of results - `Vec<Result<T, E>>` -
can be turned into a result with a vector - `Result<Vec<T>, E>`. Once an
`Result::Err` is found, the iteration will terminate.
*/
fn fail_collect() {
    let strings = vec!["Tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}

fn partition_failures() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    // Numbers and errors are still wrapped in result, so unwrap them now
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

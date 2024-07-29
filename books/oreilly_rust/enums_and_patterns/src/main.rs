#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


fn main() {
    cstyle_enums();
    methods_on_patterns();
    array_and_slice_patterns();
    reference_patterns();
}

/**
C-style enums carry only indexes.

This example is from `std::cmp::Ordering`.
*/
#[derive(Debug)]
enum Ordering {
    Less,    // 0
    Equal,   // 1
    Greater, // 2
}

/**
We can instruct Rust which integers to use.

These 'descriminant' values must be unique.
*/
#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
    // ...
}


/**
A timestamp that has been deliberately rounded off, so our program says
something like '6 months ago', instead of 'Feb 9, 2024 at 10:19:51AM'.
*/
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}


/**
Like for structs, the compiler will implement features like `==` if asked
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

/**
Enums can have methods, too.
*/
impl TimeUnit {
    fn plural(&self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Weeks => "weeks",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(&self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

fn compare(a: i32, b: i32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if b < a {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn cstyle_enums() {
    // They only take as much space an their largest discriminant.
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2); // 404 won't fit in u8

    // Only C-style enums can be cast to integers
    assert_eq!(compare(4, 4) as u8, 1);

    // But not *from* an integer
    // error[E0606]: casting `&HttpStatus` as `u16` is invalid
    //~ dbg!(&HttpStatus::Ok as u16);

    /// We can bring them all into scope
    use HttpStatus::*;

    // You can write your own conversion function:
    // Or use `enum_primitive` crate which has a macro to build these functions
    fn http_status_from_integer(code: i32) -> Option<HttpStatus> {
        match code {
            200 => Some(Ok),
            304 => Some(NotModified),
            404 => Some(NotFound),
            _ => None,
        }
    }

    // Enums can have methods
    assert_eq!(TimeUnit::Months.plural(), "months");
    assert_eq!(TimeUnit::Months.singular(), "month");
}


fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, 1) => {
            format!("{} {} ago", 1, units.singular())
        }
        RoughTime::InThePast(units, count) => {
            format!("{} {} ago", count, units.plural())
        },
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, 1) => {
            format!("{} {} from now", 1, units.singular())
        }
        RoughTime::InTheFuture(units, count) => {
            format!("{} {} from now", count, units.plural())
        }
    }
}


fn methods_on_patterns() {
    let in_three_hours = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    assert_eq!(rough_time_to_english(in_three_hours), "3 hours from now");

    let four_score_and_seven = RoughTime::InThePast(TimeUnit::Years, (4 * 20) + 7);
    assert_eq!(rough_time_to_english(four_score_and_seven), "87 years ago");

    let next_month = RoughTime::InTheFuture(TimeUnit::Months, 1);
    assert_eq!(rough_time_to_english(next_month), "1 month from now");

    let last_week = RoughTime::InThePast(TimeUnit::Weeks, 1);
    assert_eq!(rough_time_to_english(last_week), "1 week ago");
}


fn greet_people(names: &[&str]) {
    match names {
        [] => println!("Hello, nobody"),
        [a] => println!("Hello, {}!", a),
        [a, b] => println!("Hello, {} and {}!", a, b),
        [a, .., b] => println!("Hello, everyone from {} to {}.", a, b),
    }
}


fn array_and_slice_patterns() {
    let names = ["Alyson", "Blake", "Leon", "Stella"];
    greet_people(&names);
}


#[derive(Debug)]
struct Account {
    name: String,
    language: String,
}


fn reference_patterns() {
    let acc = Account{
        name: String::from("Leon"),
        language: String::from("English")
    };

    // Matching a noncopyable value moves the values out of the structure.
    // error[E0382]: borrow of partially moved value: `acc`
    /*
    match acc {
        Account { name, language } => {     // Move happens here.
            println!("Hello {} in {}", name, language);
            println!("{:?}", acc);
        }
    }
    */

    // Borrow values instead using the `ref` keyword, `ref mut` if needed.
    match acc {
        Account { ref name, ref language } => {
            println!("Hello {} in {}", name, language);
            println!("{:?}", &acc);
        }
    }
}

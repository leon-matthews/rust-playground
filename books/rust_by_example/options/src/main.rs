#![allow(dead_code)]


fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}


struct Person {
    job: Option<Job>,
}


impl Person {
    /// Gets the area code of the phone number of the person's job, if it exists.
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }

}


#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}


#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}


/// The `?` operator returns `None` early if expression is `None`
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

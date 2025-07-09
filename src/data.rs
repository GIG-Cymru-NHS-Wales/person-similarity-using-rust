// Use LazyLock for creating a thread-safe global variable e.g. our DATA.
use std::sync::LazyLock;

// Use Mutex for thread-safe access to a variable e.g. our DATA.
use std::sync::Mutex;

// Use HashMap for storing data as key-value pairs e.g. our DATA.
use std::collections::HashMap;

// Use the Person struct.
use crate::models::person::Person;

// Create a data store as a global variable with `LazyLock` and `Mutex`.
//
// This demo implementation uses a `HashMap` for ease and speed.
// The map key is a primary key for lookup; the map value is a Person.
//
// To access data, create a thread, spawn it, and acquire the lock:
//
// ```
// async fn example() {
//     thread::spawn(move || {
//         let data = DATA.lock().unwrap();
//         …
// }).join().unwrap()
// ```

// Note: static items do not call [`Drop`] on program termination, so this won't
// be deallocated. this is fine, as the OS can deallocate the terminated program
// faster than we can free memory but tools like valgrind might report "memory
// leaks" as it isn't obvious this is intentional.

pub static DATA: LazyLock<Mutex<HashMap<String, Person>>> = LazyLock::new(|| { 
    Mutex::new(HashMap::from([
        (
            String::from("1"),
            Person {
                id: String::from("1"),
                given_name: Some(String::from("Alice")), 
                family_name: Some(String::from("Anderson")),
                birth_date_year: None,
                birth_date_month: None,
                birth_date_month_day: None,
                primary_email: None,
                primary_phone: None,
                note: None,
            },
        ),
        (
            String::from("2"),
            Person { 
                id: String::from("2"),
                given_name: Some(String::from("Bob")), 
                family_name: Some(String::from("Brown")),
                birth_date_year: None,
                birth_date_month: None,
                primary_email: None,
                primary_phone: None,
                birth_date_month_day: None,
                note: None,
            },
        ),
    ]))
});

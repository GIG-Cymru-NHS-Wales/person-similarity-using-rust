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
            String::from("cc1143129505d87f5f0a044b7dbef236"),
            Person {
                id: String::from("cc1143129505d87f5f0a044b7dbef236"),
                given_name: Some(String::from("Alice")), 
                family_name: Some(String::from("Adams")),
                birth_date_year: None,
                birth_date_month: None,
                birth_date_month_day: None,
                primary_email: Some(String::from("alice.adams@example.com")),
                primary_phone: Some(String::from("3787581685")),
                note: Some(String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")),
            },
        ),
        (
            String::from("925561d3c5d097b690d029ef03d08721"),
            Person { 
                id: String::from("925561d3c5d097b690d029ef03d08721"),
                given_name: Some(String::from("Bob")), 
                family_name: Some(String::from("Brown")),
                birth_date_year: None,
                birth_date_month: None,
                birth_date_month_day: None,
                primary_email: Some(String::from("bob.brown@example.com")),
                primary_phone: Some(String::from("7792181073")),
                note: Some(String::from("Vitae suscipit tellus mauris a diam maecenas sed. Nunc sed velit dignissim sodales ut eu sem integer vitae.")),
            },
        ),
        (
            String::from("ebb7f695a7301810fcd17efff78f222d"),
            Person { 
                id: String::from("ebb7f695a7301810fcd17efff78f222d"),
                given_name: Some(String::from("Carol")), 
                family_name: Some(String::from("Clark")),
                birth_date_year: None,
                birth_date_month: None,
                birth_date_month_day: None,
                primary_email: Some(String::from("carol.clark@example.com")),
                primary_phone: Some(String::from("6955100299")),
                note: Some(String::from("Blandit libero volutpat sed cras. A cras semper auctor neque vitae tempus quam pellentesque.")),
            },
        ),
        (
            String::from("d8913a341ff72be5a6716d90c46a29da"),
            Person { 
                id: String::from("d8913a341ff72be5a6716d90c46a29da"),
                given_name: Some(String::from("David")), 
                family_name: Some(String::from("Davis")),
                birth_date_year: None,
                birth_date_month: None,
                birth_date_month_day: None,
                primary_email: Some(String::from("david.davis@example.com")),
                primary_phone: Some(String::from("9995622828")),
                note: Some(String::from("Quis eleifend quam adipiscing vitae. Quisque non tellus orci ac auctor augue mauris augue neque. Lacinia quis vel eros donec.")),
            },
        ),
        (
            String::from("6eeb89a7967a5f08851290092f9e3c2a"),
            Person { 
                id: String::from("6eeb89a7967a5f08851290092f9e3c2a"),
                given_name: Some(String::from("Eve")), 
                family_name: Some(String::from("Evans")),
                birth_date_year: None,
                birth_date_month: None,
                birth_date_month_day: None,
                primary_email: Some(String::from("eve.evans@example.com")),
                primary_phone: Some(String::from("8187236185")),
                note: Some(String::from("Lacus suspendisse faucibus interdum posuere. Malesuada fames ac turpis egestas maecenas. Adipiscing tristique risus nec feugiat.")),
            },
        ),
        (
            String::from("acefb313c39ca3cdeea597e08204cc0f"),
            Person { 
                id: String::from("acefb313c39ca3cdeea597e08204cc0f"),
                given_name: Some(String::from("Frank")), 
                family_name: Some(String::from("Franklin")),
                birth_date_year: None,
                birth_date_month: None,
                birth_date_month_day: None,
                primary_email: Some(String::from("frank.franklin@example.com")),
                primary_phone: Some(String::from("9104733641")),
                note: Some(String::from("Etiam ut feugiat nibh. Suspendisse at scelerisque lectus, ut rutrum purus. Nulla non mattis mauris. In gravida risus in ipsum venenatis feugiat quis luctus dui.")),
            },
        ),
    ]))
});

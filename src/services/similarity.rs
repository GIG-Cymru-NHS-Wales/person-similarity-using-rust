use crate::models::person::Person;

fn similarity(input: (Person, Person)) -> f64 {
    let (a, b) = input;
    let mut x: f64 = 0.0;
    if let Some(a) = a.given_name && let Some(b) = b.given_name {
        if a == b { 
            x += 1.0 
        }
    }
    if let Some(a) = a.family_name && let Some(b) = b.family_name {
        if a == b { 
            x += 1.0 
        }
    }
    if let Some(a) = a.primary_email && let Some(b) = b.primary_email {
        if a == b { 
            x += 1.0 
        }
    }
    if let Some(a) = a.primary_phone && let Some(b) = b.primary_phone {
        if a == b { 
            x += 1.0 
        }
    }
    x / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let a = Person {
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Anderson")),
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: Some(String::from("0")),
            note: None,
        };
        let b = Person { 
            id: String::from("0"),
            given_name: Some(String::from("Bob")), 
            family_name: Some(String::from("Brown")),
            birth_date_year: None,
            birth_date_month: None,
            primary_email: Some(String::from("bob@example.com")),
            primary_phone: Some(String::from("1")),
            birth_date_month_day: None,
            note: None,
        };
        assert_eq!(similarity((a, b)), 0.0);
    }

    #[test]
    fn test_1() {
        let a = Person {
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Anderson")),
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: Some(String::from("0")),
            note: None,
        };
        let b = Person { 
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Anderson")),
            birth_date_year: None,
            birth_date_month: None,
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: Some(String::from("0")),
            birth_date_month_day: None,
            note: None,
        };
        assert_eq!(similarity((a, b)), 1.0);
    }

    #[test]
    fn test_given_name() {
        let a = Person {
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Anderson")),
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: Some(String::from("0")),
            note: None,
        };
        let b = Person { 
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Brown")),
            birth_date_year: None,
            birth_date_month: None,
            primary_email: Some(String::from("bob@example.com")),
            primary_phone: Some(String::from("1")),
            birth_date_month_day: None,
            note: None,
        };
        assert_eq!(similarity((a, b)), 0.25);
    }

    #[test]
    fn test_family_name() {
        let a = Person {
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Anderson")),
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: Some(String::from("0")),
            note: None,
        };
        let b = Person { 
            id: String::from("0"),
            given_name: Some(String::from("Bob")), 
            family_name: Some(String::from("Anderson")),
            birth_date_year: None,
            birth_date_month: None,
            primary_email: Some(String::from("bob@example.com")),
            primary_phone: Some(String::from("1")),
            birth_date_month_day: None,
            note: None,
        };
        assert_eq!(similarity((a, b)), 0.25);
    }

}
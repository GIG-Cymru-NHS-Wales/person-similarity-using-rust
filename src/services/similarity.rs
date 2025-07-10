use crate::models::person::Person;

pub const GIVEN_NAME_EQ: f64 = 0.8;
pub const FAMILY_NAME_EQ: f64 = 1.0;
pub const BIRTH_DATE_YEAR_EQ: f64 = 0.3;
pub const BIRTH_DATE_MONTH_EQ: f64 = 0.2;
pub const BIRTH_DATE_MONTH_DAY_EQ: f64 = 0.1;
pub const PRIMARY_PHONE_EQ: f64 = 0.6;
pub const PRIMARY_EMAIL_EQ: f64 = 0.7;

pub const SIMILARITY_MAX: f64 = 
    GIVEN_NAME_EQ +
    FAMILY_NAME_EQ +
    BIRTH_DATE_YEAR_EQ +
    BIRTH_DATE_MONTH_EQ +
    BIRTH_DATE_MONTH_DAY_EQ +
    PRIMARY_PHONE_EQ +
    PRIMARY_EMAIL_EQ;

/// Calculate the similarity probability of two persons.
/// 
/// This function compares these fields:
/// 
/// - Given name
/// - Family name
/// - Birth date year, month, month day
/// - Primary email
/// - Primary phone
/// 
/// The text fields are compared using the function [similarity_of_strings].
/// 
/// The numeric fields are compared using equality.
/// 
pub fn similarity_of_persons(input: (Person, Person)) -> f64 {
    let (a, b) = input;
    let mut max: f64 = 0.0;
    let mut x: f64 = 0.0;
    if let Some(a) = a.given_name && let Some(b) = b.given_name {
        max += GIVEN_NAME_EQ;
        x += similarity_of_strings((&a, &b)) * GIVEN_NAME_EQ;
    }
    if let Some(a) = a.family_name && let Some(b) = b.family_name {
        max += FAMILY_NAME_EQ;
        x += similarity_of_strings((&a, &b)) * FAMILY_NAME_EQ;
    }
    if let Some(a) = a.birth_date_year && let Some(b) = b.birth_date_year {
        max += BIRTH_DATE_YEAR_EQ;
        if a == b {
            x += BIRTH_DATE_YEAR_EQ;
        }
    }
    if let Some(a) = a.birth_date_month && let Some(b) = b.birth_date_month {
        max += BIRTH_DATE_MONTH_EQ;
        if a == b {
            x += BIRTH_DATE_MONTH_EQ;
        }
    }
    if let Some(a) = a.birth_date_month_day && let Some(b) = b.birth_date_month_day {
        max += BIRTH_DATE_MONTH_DAY_EQ;
        if a == b {
            x += BIRTH_DATE_MONTH_DAY_EQ
        }
    }
    if let Some(a) = a.primary_phone && let Some(b) = b.primary_phone {
        max += PRIMARY_PHONE_EQ;
        x += similarity_of_strings((&a, &b)) * PRIMARY_PHONE_EQ;
    }
    if let Some(a) = a.primary_email && let Some(b) = b.primary_email {
        max += PRIMARY_EMAIL_EQ;
        x += similarity_of_strings((&a, &b)) * PRIMARY_EMAIL_EQ;
    }
    x / ((max + SIMILARITY_MAX) / 2.0)
}

/// Calculate the similarity of two strings.
/// 
/// This implementation uses:
/// 
/// - If either string is blank, then return 0.0 meaning no similarity.
/// 
/// - If the strings are equal, then return 1.0 meaning identical similarity.
/// 
/// - Otherwise, use the heuristic below.
/// 
/// The heuristic is an average of theses string comparison algorithms:
/// 
/// - [Jaro-Winkler distance](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)
/// 
/// - [Damerau–Levenshtein distance](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)
/// 
/// - Sørensen-Dice coefficient: (https://en.wikipedia.org/wiki/Dice-S%C3%B8rensen_coefficient)
/// 
pub fn similarity_of_strings(input: (&str, &str)) -> f64 {
    let (a, b) = input;
    if a == "" || b == "" { 
        0.0 
    }
    else 
    if a == b { 
        1.0
    }
    else {
        (
            strsim::jaro_winkler(a, b) + 
            strsim::normalized_damerau_levenshtein(a, b) +
            strsim::sorensen_dice(a, b) 
        ) / 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let a = Person {
            id: String::from("0"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        let b = Person { 
            id: String::from("1"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            primary_email: None,
            primary_phone: None,
            birth_date_month_day: None,
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), 0.0);
    }

    #[test]
    fn test_min() {
        let a = Person {
            id: String::from("0"),
            given_name: Some(String::from("aaa")), 
            family_name: Some(String::from("aaa")),
            birth_date_year: Some(1999),
            birth_date_month: Some(12),
            birth_date_month_day: Some(31),
            primary_email: Some(String::from("aaa")),
            primary_phone: Some(String::from("111")),
            note: None,
        };
        let b = Person { 
            id: String::from("1"),
            given_name: Some(String::from("bbb")), 
            family_name: Some(String::from("bbb")),
            birth_date_year: Some(2022),
            birth_date_month: Some(2),
            birth_date_month_day: Some(28),
            primary_email: Some(String::from("bbb")),
            primary_phone: Some(String::from("222")),
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), 0.0);
    }

    #[test]
    fn test_max() {
        let a = Person {
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Adams")),
            birth_date_year: Some(1999),
            birth_date_month: Some(12),
            birth_date_month_day: Some(31),
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: Some(String::from("3787581685")),
            note: None,
        };
        let b = Person { 
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Adams")),
            birth_date_year: Some(1999),
            birth_date_month: Some(12),
            birth_date_month_day: Some(31),
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: Some(String::from("3787581685")),
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), 1.0);
    }

    #[test]
    fn test_perturbations() {
        let a = Person {
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: Some(String::from("Adams")),
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: Some(String::from("3787581685")),
            note: None,
        };
        let b = Person { 
            id: String::from("0"),
            given_name: Some(String::from("lAice")), 
            family_name: Some(String::from("dAams")),
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: Some(String::from("laice@example.com")),
            primary_phone: Some(String::from("7387581685")),
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), 0.7415945790080739); // empirical
    }

    #[test]
    fn test_given_name() {
        let a = Person {
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        let b = Person { 
            id: String::from("0"),
            given_name: Some(String::from("Alice")), 
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            primary_email: None,
            primary_phone: None,
            birth_date_month_day: None,
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), GIVEN_NAME_EQ / ((GIVEN_NAME_EQ + SIMILARITY_MAX) / 2.0));
    }

    #[test]
    fn test_family_name() {
        let a = Person {
            id: String::from("0"),
            given_name: None,
            family_name: Some(String::from("Adams")),
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        let b = Person { 
            id: String::from("0"),
            given_name: None,
            family_name: Some(String::from("Adams")),
            birth_date_year: None,
            birth_date_month: None,
            primary_email: None,
            primary_phone: None,
            birth_date_month_day: None,
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), FAMILY_NAME_EQ / ((FAMILY_NAME_EQ + SIMILARITY_MAX) / 2.0));
    }

    #[test]
    fn test_birth_date_year() {
        let a = Person {
            id: String::from("0"),
            given_name: None,
            family_name: None,
            birth_date_year: Some(1999),
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        let b = Person { 
            id: String::from("1"),
            given_name: None,
            family_name: None,
            birth_date_year: Some(1999),
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), BIRTH_DATE_YEAR_EQ  / ((BIRTH_DATE_YEAR_EQ + SIMILARITY_MAX) / 2.0) )
    }

    #[test]
    fn test_birth_date_month() {
        let a = Person {
            id: String::from("0"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: Some(12),
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        let b = Person { 
            id: String::from("1"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: Some(12),
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), BIRTH_DATE_MONTH_EQ / ((BIRTH_DATE_MONTH_EQ + SIMILARITY_MAX) / 2.0) )
    }

        #[test]
    fn test_birth_date_month_day() {
        let a = Person {
            id: String::from("0"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: Some(31),
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        let b = Person { 
            id: String::from("1"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: Some(31),
            primary_email: None,
            primary_phone: None,
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), BIRTH_DATE_MONTH_DAY_EQ / ((BIRTH_DATE_MONTH_DAY_EQ + SIMILARITY_MAX) / 2.0) )
    }

    #[test]
    fn test_primary_email() {
        let a = Person {
            id: String::from("0"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: None,
            note: None,
        };
        let b = Person { 
            id: String::from("1"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: Some(String::from("alice@example.com")),
            primary_phone: None,
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), PRIMARY_EMAIL_EQ / ((PRIMARY_EMAIL_EQ + SIMILARITY_MAX) / 2.0) )
    }

    #[test]
    fn test_primary_phone() {
        let a = Person {
            id: String::from("0"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: Some(String::from("3787581685")),
            note: None,
        };
        let b = Person { 
            id: String::from("1"),
            given_name: None,
            family_name: None,
            birth_date_year: None,
            birth_date_month: None,
            birth_date_month_day: None,
            primary_email: None,
            primary_phone: Some(String::from("3787581685")),
            note: None,
        };
        assert_eq!(similarity_of_persons((a, b)), PRIMARY_PHONE_EQ / ((PRIMARY_PHONE_EQ + SIMILARITY_MAX) / 2.0) )
    }

}
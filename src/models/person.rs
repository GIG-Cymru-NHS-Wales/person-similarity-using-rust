/// Use Deserialize to convert e.g. from request JSON into Book struct.
use serde::Deserialize;

// Demo person structure with some example fields for title and author.
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Person {
    pub id: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub birth_date_year: Option<i32>,
    pub birth_date_month: Option<i8>,
    pub birth_date_month_day: Option<i8>,
    pub primary_email: Option<String>,
    pub primary_phone: Option<String>,
    pub note: Option<String>,
}

// Display the person using any kind of reasonable format.
// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
            concat!(
                "given name: {:?}, ",
                "family name: {:?}, ",
                "birth date year: {:?}, ",
                "birth date month: {:?}, ",
                "birth date month day: {:?}, ",
                "primary email: {:?}, ",
                "primary phone: {:?}",
                "note: {:?}",
            ),
            self.given_name,
            self.family_name,
            self.birth_date_year,
            self.birth_date_month,
            self.birth_date_month_day,
            self.primary_email,
            self.primary_phone,
            self.note,
        )
    }
}

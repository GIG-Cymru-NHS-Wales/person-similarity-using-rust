use std::thread;
use crate::data::DATA;
use crate::views::html::*;

/// axum handler for "GET /persons" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get_persons() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut persons = data.values().collect::<Vec<_>>().clone();
        persons.sort_by(|a, b| 
            a.given_name.as_deref().unwrap_or("").cmp(b.given_name.as_deref().unwrap_or(""))
        );
        let table: Vec<Vec<String>> = persons.iter().map(|person| 
            vec![
                person.id.clone(),
                (match &person.given_name { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.family_name { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.birth_date_year { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.birth_date_month { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.birth_date_month_day { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.primary_email { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.primary_phone { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.note { Some(x) => x.to_string(), None => "".to_string() }),
            ]
        ).collect();
        html_table_tag(table)
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "GET /persons/{id}/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_persons_id_form(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(person) => format!(
                concat!(
                    "<form method=\"post\" action=\"/persons/{}/form\">\n",
                    "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                    "<p><input name=\"given_name\" value=\"{}\"></p>\n",
                    "<p><input name=\"family_name\" value=\"{}\"></p>\n",
                    "<p><input name=\"birth_date_year\" value=\"{}\"></p>\n",
                    "<p><input name=\"birth_date_month\" value=\"{}\"></p>\n",
                    "<p><input name=\"birth_date_month_day\" value=\"{}\"></p>\n",
                    "<p><input name=\"primary_email\" value=\"{}\"></p>\n",
                    "<p><input name=\"primary_phone\" value=\"{}\"></p>\n",
                    "<p><input name=\"note\" value=\"{}\"></p>\n",
                    "<input type=\"submit\" value=\"Save\">\n",
                    "</form>\n"
                ),
                &person.id, 
                &person.id, 
                (match &person.given_name { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.family_name { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.birth_date_year { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.birth_date_month { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.birth_date_month_day { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.primary_email { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.primary_phone { Some(x) => x.to_string(), None => "".to_string() }),
                (match &person.note { Some(x) => x.to_string(), None => "".to_string() }),
            ),
            None => format!("<p>Person id {} not found</p>", id),
        }
    })
    .join()
    .unwrap()
    .into()
}

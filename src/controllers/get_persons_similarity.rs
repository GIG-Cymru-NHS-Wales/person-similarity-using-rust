use crate::models::person::Person;

type PersonsPair = (Person, Person);

pub async fn persons_similarity(
    axum::extract::Json(pair): axum::extract::Json<PersonsPair>
) -> String{
    format!("Get demo JSON data: {:?}", pair)
}

use rocket::local::blocking::Client;

use crate::data::read_access;
use crate::util::month_converter;


#[get("/year_and_month_with_most_hurricanes")]
pub fn execute() -> String {
    let year_and_month_with_most_hurricanes = read_access::get_year_and_month_with_most_hurricanes();

    let year = year_and_month_with_most_hurricanes.0;
    let month_name = month_converter::get_month_name_from_index(year_and_month_with_most_hurricanes.1);
    let occurances = year_and_month_with_most_hurricanes.2;

    return format!("The most common month is {} {} with an gigantic amount of {} hurricanes!", month_name, year, occurances)
}

#[test]
fn test() {
    let client = Client::tracked(crate::rocket()).unwrap();
    let response = client.get("/hurricanes/year_and_month_with_most_hurricanes").dispatch();
    assert_eq!(response.into_string(), Some("The most common month is Aug 2012 with an gigantic amount of 8 hurricanes!".into()));
}
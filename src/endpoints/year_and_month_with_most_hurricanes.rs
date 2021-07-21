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
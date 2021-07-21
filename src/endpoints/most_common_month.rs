use crate::data::read_access;
use crate::util::month_converter;

#[get("/most_common_month")]
pub fn execute() -> String {
    let most_common_month = read_access::get_most_common_month();

    let month_name = month_converter::get_month_name_from_index(most_common_month.0);

    return format!("The most common month is {} with an average of {} hurricanes!", month_name, most_common_month.1)
}
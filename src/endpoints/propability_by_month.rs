use crate::data::read_access;
use crate::util::month_converter;


#[get("/propability_by_month?<month>")]
pub fn execute(month: u8) -> String {
    if month < 1 || month > 12 { 
        return "Invalid Input!".to_string();
    }
    
    let month_index = month - 1;
    let month_name = month_converter::get_month_name_from_index(month_index);
    let propability = match read_access::get_propability_by_month(month_index) {
        Ok(data) => data,
        Err(_) => return "Failed to get propability for the requested month!".to_string()
    };
    let readable_propability = format!("{:.2}%", (propability * 100.0));

    return format!("The hurricane propability for the month {} is {}!", month_name, readable_propability);
}
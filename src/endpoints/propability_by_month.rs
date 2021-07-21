
#[get("/propability_by_month?<month>")]
pub fn execute(month: u8) -> String {
    if month < 1 || month > 12 { 
        "Invalid Input!".to_string()
    } else { 
        format!("propability_by_month for {}", month)
    }
}